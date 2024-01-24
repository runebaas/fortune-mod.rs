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
use regex::Regex;

pub fn filter(fortunes: Vec<String>, params: &Parameters) -> Vec<String> {
    let pattern = if params.case_insensitive {
        format!("(?i){}", &params.pattern)
    } else {
        params.pattern.clone()
    };
    let regex = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(err) => panic!(
            "Pattern \"{}\" is not a valid regex pattern: {}",
            params.pattern, err
        ),
    };

    fortunes
        .into_iter()
        .filter(|x| regex.is_match(x))
        .collect::<Vec<String>>()
}
