use std::fs;
use rand::prelude::*;
use clap::{Arg, App};

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
            .default_value("160")
            .help("Set the longest fortune length (in characters) considered to be 'short'. All fortunes longer than this are considered 'long'.")
            .takes_value(true)
            .value_name("length"));
    let matches = app.get_matches();

    // get fortunes
    let filename = matches.value_of("file").unwrap().to_owned();
    let fortune_file = fs::read_to_string(filename).expect("Cannot read fortune file");
    let fortunes: Vec<&str> = fortune_file.split('%').collect();

    // filter by max length
    let max_length = matches
        .value_of("length")
        .unwrap()
        .parse::<usize>()
        .expect("Length is not a valid number");
    let filtered = fortunes
        .into_iter()
        .filter(|x| x.replace(" ", "").len() < max_length)
        .collect::<Vec<&str>>();

    // get a random one
    let total_fortunes = filtered.len();
    let random_fortune = rand::thread_rng().gen_range(0, total_fortunes);

    // print the fortune
    let the_fortune = filtered.get(random_fortune).unwrap().trim();
    println!("{}", the_fortune);
}
