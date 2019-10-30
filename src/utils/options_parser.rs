use clap::App;

pub fn parse(app: App) -> Parameters {
    let matches = app.get_matches();
    let options: Parameters = Parameters {
        filename: matches.value_of("file").unwrap().to_owned(),
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
    };

    options
}

pub struct Parameters {
    pub filename: String,

    pub length: usize,
    pub long_fortunes: bool,
    pub short_fortunes: bool,

    pub wait: bool,

    pub pattern: String,
    pub case_insensitive: bool,
}
