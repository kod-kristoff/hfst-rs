use clap::{App, Arg};
use clap::{crate_authors, crate_description, crate_name, crate_version};


pub fn app() -> App<'static, 'static>  {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("INFILE")
             .help("the input to work on"))
        // print_common_program_options(message_out);
        // print_common_unary_program_options(message_out); 
        .arg(Arg::with_name("epsilon")
                .long("epsilon")
                .takes_value(true)
                .value_name("EPS")
                .help("map EPS as zero"))
}
