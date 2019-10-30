use std::{fs, path, process};

pub fn get_all(filename: String) -> Vec<String> {
    if !path::Path::new(&filename).exists() {
        println!("File '{}' does not found", filename);
        process::exit(1);
    }
    let fortune_file = fs::read_to_string(filename).expect("Cannot read fortune file");
    let fortunes: Vec<String> = fortune_file.split('%').map(ToOwned::to_owned).collect();

    fortunes
}
