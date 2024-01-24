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

use rand::*;

pub fn get_random_from_vec(vector: Vec<String>) -> Option<String> {
    let length = vector.len();
    if length == 0 {
        return None;
    }
    let random_element = thread_rng().gen_range(0..length);

    Some(vector.get(random_element).unwrap().to_owned())
}

pub fn own_vec(input: Vec<&str>) -> Vec<String> {
    input.into_iter().map(ToOwned::to_owned).collect()
}
