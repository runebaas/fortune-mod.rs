use std::fs;
use rand::prelude::*;
use clap::{Arg, App};

struct Options {
    filename: String,
    length: usize
}

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
    let options = parse_options(app);

    // get fortunes
    let fortunes = get_fortunes(options.filename.clone());

    // filter by max length
    let filtered = fortunes
        .into_iter()
        .filter(|x| x.replace(" ", "").len() < options.length)
        .collect::<Vec<String>>();

    // get a random one
    let the_fortune = get_random_fortune(filtered);
    println!("{}", the_fortune);
}

fn parse_options(app: App) -> Options {
    let matches = app.get_matches();
    let options: Options = Options {
        filename: matches.value_of("file").unwrap().to_owned(),
        length: matches.value_of("length").unwrap().parse::<usize>().expect("Length is not a valid number")
    };

    return options;
}

fn get_fortunes(filename: String) -> Vec<String> {
    let fortune_file = fs::read_to_string(filename).expect("Cannot read fortune file");
    let fortunes: Vec<String> = fortune_file.split('%').map(ToOwned::to_owned).collect();

    return fortunes;
}

fn get_random_fortune(fortunes: Vec<String>) -> String {
    let total_fortunes = fortunes.len();
    let random_fortune = rand::thread_rng().gen_range(0, total_fortunes);

    return fortunes.get(random_fortune).unwrap().trim().to_owned();
}
