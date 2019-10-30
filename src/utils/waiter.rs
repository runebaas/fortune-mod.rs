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
