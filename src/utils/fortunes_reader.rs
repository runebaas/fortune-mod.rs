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
    let locations = get_paths();
    let mut all_files: Vec<String> = vec![];

    for loc in locations {
        let full_path = path::Path::new(&loc);
        if full_path.exists() {
            if let Ok(files) = fs::read_dir(full_path.canonicalize().unwrap()) {
                for file in files {
                    if let Ok(f) = file {
                        if is_fortfile(&f.path().to_str().unwrap()) {
                            all_files.push(f.path().to_str().unwrap().to_owned());
                        }
                    }
                }
            }
        }
    }

    all_files
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
    own_vec(vec!["./fortunes", "./cookies", "/lib/share/games/fortunes"])
}

pub struct Cookie {
    pub name: String,
    pub fortunes: Vec<String>,
}
