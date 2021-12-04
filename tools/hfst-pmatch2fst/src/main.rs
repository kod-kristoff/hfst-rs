use std::fs;
use std::io;

use log::LevelFilter;

use args::Args;
use hfst;
use hfst_output;
use pmatch;

mod args;
mod app;


fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{:#}", err);
        std::process::exit(2);
    }
}

fn try_main(args: Args) -> eyre::Result<()> {
    let log_level = if args.quiet() {
        LevelFilter::Error
    } else if args.debug() {
        LevelFilter::Debug
    } else if args.verbose() {
        LevelFilter::Info
    } else {
        LevelFilter::Trace
        //LevelFilter::Warn
    };
    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    let reader: Box<dyn io::Read> = if args.input() == "<stdin>" {
        let stdin = io::stdin();
        Box::new(stdin)
    } else {
        let f = fs::File::open(args.input())?;
        Box::new(f)
    };
    log::info!("Reading from '{}', writing to '{}'",
        args.input(),
        args.output(),
    );
    let outstream = hfst_output::HfstOLWOutputStream::from_name(args.output());
    let mut comp = pmatch::PmatchCompiler::new();
    comp.set_verbose(args.verbose());

    process_stream(
        comp,
        reader,
        outstream
    )
}

fn process_stream(
    comp: pmatch::PmatchCompiler,
    mut reader: Box<dyn io::Read>, 
    outstream: hfst_output::HfstOLWOutputStream
) -> eyre::Result<()> {
    log::trace!("Processing stream...");
    let mut file_contents = String::new();
    reader.read_to_string(&mut file_contents)?;
    let definitions = comp.compile(&file_contents)?;
    log::debug!("definitions:");
    for (name, transducer) in &definitions {
        log::debug!("  {}, {}", name, transducer.name());
    }
    Ok(())
}
