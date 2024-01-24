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

use crate::utils::helpers::{get_random_from_vec, own_vec};
use std::{fs, path, process};

pub fn get_random_cookie() -> Option<Cookie> {
    let fortune_files = find_fortune_files();
    if let Some(selected_file) = get_random_from_vec(fortune_files) {
        let fortunes = read_fortune_file(&selected_file);
        let cookie = Cookie {
            name: selected_file,
            fortunes,
        };
        Some(cookie)
    } else {
        None
    }
}

fn find_fortune_files() -> Vec<String> {
    get_paths()
        .into_iter()
        .map(|loc| path::Path::new(&loc).to_owned())
        .filter(|loc| loc.exists())
        .filter_map(|full_path| fs::read_dir(full_path.canonicalize().unwrap()).ok())
        .flat_map(|files| {
            files
                .filter_map(|file| file.ok())
                .map(|file| file.path().to_str().unwrap().to_owned())
        })
        .filter(|file_path| is_fortfile(file_path))
        .collect::<Vec<_>>()
}

fn is_fortfile(path: &str) -> bool {
    !path.contains('.')
}

fn read_fortune_file(filename: &str) -> Vec<String> {
    if !path::Path::new(&filename).exists() {
        println!("File '{}' does not found", filename);
        process::exit(1);
    }
    let fortune_file = fs::read_to_string(filename).expect("Cannot read fortune file");

    fortune_file.split('%').map(ToOwned::to_owned).collect()
}

#[cfg(windows)]
fn get_paths() -> Vec<String> {
    own_vec(vec!["./fortunes", "./cookies"])
}

#[cfg(unix)]
fn get_paths() -> Vec<String> {
    own_vec(vec!["./fortunes", "/usr/share/fortune", "/usr/share/games/fortunes"])
}

pub struct Cookie {
    pub name: String,
    pub fortunes: Vec<String>,
}
