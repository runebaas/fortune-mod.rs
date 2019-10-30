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
    x.replace(" ", "").len() < max_length
}

fn filter_short(x: &str, max_length: usize) -> bool {
    x.replace(" ", "").len() > max_length
}
