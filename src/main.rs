mod filters;
mod utils;

use crate::utils::*;
use clap::{App, Arg};
use rand::prelude::*;

fn main() {
    // specify app
    let app = App::new("Fortune.rs")
        .version("0.1")
        .author("Daan Boerlage <runebaas@gmail.com>")
        .about("When fortune is run with no arguments it prints out a random epigram. Epigrams are divided into several categories.")
        .arg(Arg::with_name("file")
            .short("F")
            .long("file")
            .default_value("fortunes")
            .help("The fortune file")
            .takes_value(true)
            .value_name("file"))
        .arg(Arg::with_name("length")
            .short("n")
            .long("length")
            .default_value("160")
            .help("Set the longest fortune length (in characters) considered to be 'short'. All fortunes longer than this are considered 'long'.")
            .takes_value(true)
            .value_name("length"))
        .arg(Arg::with_name("long")
            .short("l")
            .long("long")
            .help("only return long fortunes")
            .takes_value(false)
            .conflicts_with("short"))
        .arg(Arg::with_name("short")
            .short("s")
            .long("short")
            .help("only return short fortunes")
            .takes_value(false)
            .conflicts_with("long"))
        .arg(Arg::with_name("wait")
            .short("w")
            .long("wait")
            .help("Wait before termination for an amount of time calculated from the number of characters in the message. (20 characters = 1 second, mim 6 seconds)"))
        .arg(Arg::with_name("pattern")
            .short("m")
            .help("Print out all fortunes which match the basic regular expression pattern.")
            .takes_value(true))
        .arg(Arg::with_name("case_insensitive")
            .short("i")
            .help("Ignore case for -m patterns. ")
            .takes_value(false));
    let options = options_parser::parse(app);

    // get fortunes
    let mut fortunes = fortunes_reader::get_all(options.filename.clone());

    // filter by max length
    if options.short_fortunes || options.long_fortunes {
        fortunes = crate::filters::length::filter(fortunes, &options);
    }

    // apply the pattern
    if options.pattern != String::new() {
        fortunes = crate::filters::pattern::filter(fortunes, &options)
    }

    // get a random one
    let the_fortune = get_random_fortune(fortunes);

    // print it ✨
    println!("{}", the_fortune);

    if options.wait {
        waiter::wait(the_fortune)
    }
}

fn get_random_fortune(fortunes: Vec<String>) -> String {
    let total_fortunes = fortunes.len();
    if total_fortunes == 0 {
        return "No Fortunes found with these parameters...".to_owned();
    }
    let random_fortune = rand::thread_rng().gen_range(0, total_fortunes);

    fortunes.get(random_fortune).unwrap().trim().to_owned()
}
