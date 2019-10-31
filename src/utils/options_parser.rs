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
