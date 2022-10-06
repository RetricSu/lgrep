mod lib;

use bm_search::search;
use clap::{App, Arg, SubCommand};
use lib::metadata::Metadata;

fn main() {
    let metadata = Metadata::current();
    let matches = App::new(metadata.name)
        .version(metadata.version.to_string().as_str())
        .author(metadata.author.as_str())
        .about(metadata.description.as_str())
        .arg(
            Arg::with_name("text")
                .help("Sets the text to search from")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("pattern")
                .help("Sets the pattern text to search")
                .required(true)
                .index(2),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let pattern = matches.value_of("pattern").unwrap();

    let result = search(text, pattern);
    for res in result {
        println!("'{}' appears at position {}", pattern, res);
    }
}
