use std::collections::HashSet;
use regex::Regex;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;
mod unittests;
pub mod write_completions;
mod strwriter;
mod get_metadata;
mod yaml_descent;
mod get_description;

pub fn keys_starting_with<'a>(prefix : &str, map : &'a Hash, ignores: &HashSet<String>) -> Vec<&'a Yaml> {
    let mut keys = Vec::new();

    for (key, _) in map {
        if let Some(key_str) = key.as_str() {
            if ignores.contains(&key_str.to_string()) {
                continue ;
            }
            if key_str.starts_with(prefix) {
                keys.push(key);
            }
        }
    }
    keys.sort();
    keys
}

pub fn yaml_descend<'a>(path: &str, yaml : &'a Yaml) -> Result<&'a Yaml, String> {

    let mut current = yaml;
    let re = Regex::new(r"([^.\[\]]+)(\.)?|(?:\[(\d+)\]?|(\[$))?").unwrap();
    for cap in re.captures_iter(path) {


    }


    Ok(current)
}