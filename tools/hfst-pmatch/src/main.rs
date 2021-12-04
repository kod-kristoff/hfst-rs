use std::fs;
use std::io;

use eyre::{WrapErr, Result};
use log::LevelFilter;

use pmatch::{PmatchContainer};
use pmatch_serialize::PmatchHfst3;

mod args;
mod app;
use args::Args;

//type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{:#}", err);
        std::process::exit(2);
    }
}

fn try_main(args: Args) -> eyre::Result<()> {
    let log_level = LevelFilter::Trace;

    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    log::info!("Using transducer from file '{}'", args.transducer());
    let container = PmatchHfst3::from_file(
        fs::File::open(args.transducer())
            .wrap_err_with(|| format!("Failed to read transducer from '{:}'", args.transducer()))?
    )?;

    let mut reader: Box<dyn io::BufRead> = if args.input() == "<stdin>" {
        log::debug!("Reading input from <stdin>");
        let stdin = io::stdin();
        Box::new(io::BufReader::new(stdin))
    } else {
        log::debug!("Reading input from '{}'", args.input());
        let f = fs::File::open(args.input())?;
        Box::new(io::BufReader::new(f))
    };
    let mut writer: Box<dyn io::Write> = if args.output() == "<stdout>" {
        log::debug!("Writing output to <stdout>");
        let stdout = io::stdout();
        Box::new(io::BufWriter::new(stdout))
    } else {
        log::debug!("Writing output to '{}'", args.output());
        let f = fs::File::open(args.output())?;
        Box::new(io::BufWriter::new(f))
    };
    process_input(
        container,
        reader,
        writer,
    )
}

fn process_input(
    container: PmatchContainer,
    reader: Box<dyn io::BufRead>,
    mut outstream: Box<dyn io::Write>,
) -> Result<()> {
    use std::io::BufRead;

    log::trace!("Called process_input with container={:?}", &container);
    for line in reader.lines() {
        match_and_print(&container, &line?, &mut outstream)?;    
    }
    Ok(())
}

fn match_and_print(
    container: &PmatchContainer,
    line: &str,
    outstream: &mut Box<dyn io::Write>,
) -> Result<()> {
    log::trace!("Called match_and_print with line='{}'", line);
    writeln!(outstream, "{}", container.pmatch(line))?;
    Ok(())
}
