use std::{fs, thread, time};
use std::convert::TryFrom;
use rand::prelude::*;
use clap::{Arg, App};

struct Options {
    filename: String,
    length: usize,
    long_fortunes: bool,
    wait: bool
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
            .long("length")
            .default_value("160")
            .help("Set the longest fortune length (in characters) considered to be 'short'. All fortunes longer than this are considered 'long'.")
            .takes_value(true)
            .value_name("length"))
        .arg(Arg::with_name("long")
            .short("l")
            .long("long")
            .help("only return long fortunes")
            .takes_value(false))
        .arg(Arg::with_name("wait")
            .short("w")
            .long("wait")
            .help("Wait before termination for an amount of time calculated from the number of characters in the message. (20 characters = 1 second, mim 6 seconds)"));
    let options = parse_options(app);

    // get fortunes
    let fortunes = get_fortunes(options.filename.clone());

    // filter by max length
    let filter_fn = if options.long_fortunes { filter_short } else { filter_long };
    let filtered = fortunes
        .into_iter()
        .filter(|x| filter_fn(x, options.length))
        .collect::<Vec<String>>();

    // get a random one
    let the_fortune = get_random_fortune(filtered);
    println!("{}", the_fortune);

    if options.wait {
        wait(the_fortune)
    }
}

fn filter_long(x: &str, max_length: usize) -> bool {
    return x.replace(" ", "").len() < max_length;
}

fn filter_short(x: &str, max_length: usize) -> bool {
    return x.replace(" ", "").len() > max_length;
}

fn parse_options(app: App) -> Options {
    let matches = app.get_matches();
    let options: Options = Options {
        filename: matches.value_of("file").unwrap().to_owned(),
        length: matches.value_of("length").unwrap().parse::<usize>().expect("Length is not a valid number"),
        long_fortunes: matches.is_present("long"),
        wait: matches.is_present("wait")
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

fn wait(fortune: String) {
    let characters_per_second = 20; // as defined in the original fortune command
    let minimum_wait = 6; // seconds
    let mut time_to_wait = fortune.len() / characters_per_second;
    if time_to_wait < minimum_wait {
        time_to_wait = minimum_wait;
    }

    let fortune_length = u64::try_from(time_to_wait).unwrap();
    let time_to_wait = time::Duration::from_secs(fortune_length);

    thread::sleep(time_to_wait);
}
