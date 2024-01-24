/*
 * Copyright (C) 2024  Daan Boerlage <d@boerlage.me>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod filters;
mod utils;

use crate::utils::*;
use clap::{App, Arg};
use std::path::Path;

fn main() {
    // specify app
    let app = App::new("Fortune.rs")
        .version("0.1")
        .author("Daan Boerlage <runebaas@gmail.com>")
        .about("When fortune is run with no arguments it prints out a random epigram. Epigrams are divided into several categories.")
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
            .takes_value(false))
        .arg(Arg::with_name("show_cookie")
            .short("c")
            .help("Show the cookie file from which the fortune came.")
            .takes_value(false));
    let options = options_parser::parse(app);

    // get fortunes
    let cookie = match fortunes_reader::get_random_cookie() {
        Some(fortunes) => fortunes,
        None => {
            println!("No fortune files found");
            std::process::exit(1);
        }
    };

    let mut fortunes = cookie.fortunes;

    // filter by max length
    if options.short_fortunes || options.long_fortunes {
        fortunes = crate::filters::length::filter(fortunes, &options);
    }

    // apply the pattern
    if options.pattern != String::new() {
        fortunes = crate::filters::pattern::filter(fortunes, &options)
    }

    // get a random one
    let the_fortune = match helpers::get_random_from_vec(fortunes) {
        Some(f) => f,
        None => {
            println!("No fortunes found with these parameters");
            std::process::exit(1);
        }
    };

    // reveal the cookie
    if options.show_cookie {
        let cookie_file = Path::new(&cookie.name).file_name().unwrap().to_str().unwrap();
        println!("({})\n%", cookie_file);
    }

    // print it ✨
    println!("{}", the_fortune);

    if options.wait {
        waiter::wait(the_fortune)
    }
}
