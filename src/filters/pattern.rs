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
