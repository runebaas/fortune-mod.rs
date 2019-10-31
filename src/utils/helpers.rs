use rand::*;

pub fn get_random_from_vec(vector: Vec<String>) -> Option<String> {
    let length = vector.len();
    if length == 0 {
        return None;
    }
    let random_element = rand::thread_rng().gen_range(0, length);

    Some(vector.get(random_element).unwrap().trim().to_owned())
}

pub fn own_vec(input: Vec<&str>) -> Vec<String> {
    input.into_iter().map(ToOwned::to_owned).collect()
}
