use args::Args;
use hfst;
use pmatch;

mod args;
mod app;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

fn try_main(args: Args) -> Result<()> {

    println!("Reading from '{}', writing to '{}'",
        args.infilename(),
        args.outfilename(),
    );
    let outstream = hfst::OutputStream::from_name(args.outfilename());
    process_stream(outstream)
}

fn process_stream(
    outstream: hfst::OutputStream
) -> Result<()> {
    println!("Processing stream...");
    let comp = pmatch::PmatchCompiler::new();
    let definitions = comp.compile()
    Ok(())
}
