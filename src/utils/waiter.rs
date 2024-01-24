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

use std::convert::TryFrom;
use std::{thread, time};

pub fn wait(fortune: String) {
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
