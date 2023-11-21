extern crate core;


use clap::{App, Arg};
use i18n_compiler::i18n_compiler;

fn main() {
    let matches = App::new("i18n_compiler")
        .version("1.0")
        .author("coderlang@foxmail.com")
        .about("i18n_compiler")
        .arg(Arg::with_name("input")
            .long("input")
            .help("Specify a CSV input file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .long("output")
            .help("Specify a i18n output file")
            .takes_value(true))
        .arg(Arg::with_name("platform")
            .long("platform")
            .help("Specify a platform, eg: ios,android,web")
            .takes_value(true))
        .get_matches();

    let input = matches.value_of("input");
    let output = matches.value_of("output");
    let platform = matches.value_of("platform");

    match (input, output, platform) {
        (Some(input), Some(output), Some(platform)) => {
            i18n_compiler::i18n_compiler::compile(platform, input, output);
        }
        _ => {
            eprintln!("Usage: ./i18n_compiler --platform [ios,android,web] --input [./test/i18n.csv] --output [./test]");
        }
    }

    return ;
}

