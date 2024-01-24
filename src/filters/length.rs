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

use crate::utils::options_parser::Parameters;

pub fn filter(fortunes: Vec<String>, options: &Parameters) -> Vec<String> {
    let filter_fn = if options.long_fortunes {
        filter_short
    } else {
        filter_long
    };

    fortunes
        .into_iter()
        .filter(|x| filter_fn(x, options.length))
        .collect::<Vec<String>>()
}

fn filter_long(x: &str, max_length: usize) -> bool {
    x.replace(' ', "").len() < max_length
}

fn filter_short(x: &str, max_length: usize) -> bool {
    x.replace(' ', "").len() > max_length
}
