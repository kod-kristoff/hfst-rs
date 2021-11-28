use args::Args;

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

    println!("jq filter: {}", args.program());
    println!("Using file '{}' as input", args.input());
    Ok(())
}
