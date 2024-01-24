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

use clap::App;

pub fn parse(app: App) -> Parameters {
    let matches = app.get_matches();
    let options: Parameters = Parameters {
        length: matches
            .value_of("length")
            .unwrap()
            .parse::<usize>()
            .expect("Length is not a valid number"),
        short_fortunes: matches.is_present("short"),
        long_fortunes: matches.is_present("long"),
        wait: matches.is_present("wait"),
        pattern: match matches.value_of("pattern") {
            Some(pat) => pat.to_owned(),
            None => String::new(),
        },
        case_insensitive: matches.is_present("case_insensitive"),
        show_cookie: matches.is_present("show_cookie"),
    };

    options
}

pub struct Parameters {
    pub length: usize,
    pub long_fortunes: bool,
    pub short_fortunes: bool,

    pub wait: bool,

    pub pattern: String,
    pub case_insensitive: bool,

    pub show_cookie: bool,
}
