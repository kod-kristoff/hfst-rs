use clap::{App, Arg};
use clap::{crate_authors, crate_description, crate_name, crate_version};


pub fn app() -> App<'static, 'static>  {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("infilename")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("INFILE")
                .help("Read input transducer from INFILE"))
        .arg(Arg::with_name("outfilename")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("OUTFILE")
                .help("Write output transducer to OUTFILE"))
        .arg(Arg::with_name("verbose")
                .long("verbose")     
                .help("print verbose output (NER stages) to standard error"))
        .arg(Arg::with_name("quiet")
                .long("quiet")     
                .help("print verbose output (NER stages) to standard error"))
        .arg(Arg::with_name("debug")
                .long("debug")       
                .help("print debug output to standard error and do not remove temporary files (mainly useful for hfst-swener developers)"
        ))
        // print_common_program_options(message_out);
        // print_common_unary_program_options(message_out); 
        .arg(Arg::with_name("epsilon")
                .long("epsilon")
                .takes_value(true)
                .value_name("EPS")
                .help("map EPS as zero"))
}
