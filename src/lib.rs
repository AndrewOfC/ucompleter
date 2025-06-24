use yaml_rust::yaml::Hash;
mod unittests;
pub mod write_completions;
mod strwriter;

pub fn keys_starting_with<'a>(prefix : &str, map : &'a Hash) -> Vec<&'a str> {
    let mut keys = Vec::new();

    for (key, _) in map {
        if let Some(key_str) = key.as_str() {
            if key_str.starts_with(prefix) {
                keys.push(key_str);
            }
        }
    }
    keys.sort();
    keys
}

