//! This module provides metadata retrieval and writing functionalities for processing YAML documents.
//!
//! The main functions in this module are:
//!
//! - `get_metadata`
//! - `write_completions`

use regex::Regex;
use std::io::{Error, Write};
use yaml_rust::Yaml;

const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const PERIOD_MATCH: usize = 2 ;
const INDEX_MATCH: usize = 3 ;
const ARRAY_MATCH: usize = 4;


fn get_metadata(tree: &Yaml) -> (&Yaml, Yaml) {
    let metakey = Yaml::String("completion-metadata".to_string());
    let rootkeykey = Yaml::String("root".to_string());
    let terminuskey = Yaml::String("terminus".to_string());
    let mut terminus = Yaml::String("".to_string());
    
    let Yaml::Hash(hash) = tree else { return (tree, terminus) };
    if !hash.contains_key(&metakey) {
        return (tree, terminus);
    }
    let &Yaml::Hash(meta) = &hash.get(&metakey).unwrap() else { panic!("Metadata must be a hash"); };

    let rootkey = meta.get(&rootkeykey).expect("Metadata root key must be a string");
    let root = hash.get(rootkey).expect(format!("root '{:?}' not found", rootkey).as_str()) ;
    
    terminus = meta.get(&terminuskey).unwrap_or(&terminus).clone();

    (root, terminus)
}

pub fn write_completions<W: Write>(writer: &mut W, inputyaml: &Yaml, inputpath: &str) -> std::io::Result<()>
{
    let re = Regex::new(r"([^.\[\]]+)(\.)?|(?:\[(\d+)\]|(\[$))?").unwrap();
    let mut path = String::from("") ;
    let mut path_separator = String::from(""); // initially empty for root path

    let (mut current, terminus) = get_metadata(inputyaml);
    
    for captures in re.captures_iter(inputpath) {
        if captures.get(ARRAY_MATCH).is_some() {
            if !matches!(current, Yaml::Array(_)) {
                panic!("Attempting to use array index on non-array node");
            }
            break;  
        }
        let m0 = captures.get(WHOLE_MATCH).unwrap();
        if m0.start() == 0 && m0.end() == 0 {
            continue;
        }

        match current {
            Yaml::Hash(map) => {
                let m1 = captures.get(KEY_MATCH).unwrap();
                let key = &inputpath[m1.start() .. m1.end()] ;
                let ykey = Yaml::String(key.to_string());
                if captures.get(PERIOD_MATCH).is_some() {
                    current = map.get(&ykey).unwrap();
                    path = format!("{}{}.", path, key);
                    continue ;
                }
                let current_components = crate::keys_starting_with(key, map) ;
                if current_components.len() == 1 {
                    if !map.contains_key(&ykey) {
                        current = map.get(&Yaml::String(current_components[0].to_string())).unwrap();
                        path = format!("{}{}{}", path, path_separator, current_components[0]);
                        path_separator = String::from(".") ;
                        break ;
                    }
                    current = map.get(&ykey).unwrap();
                    if let Yaml::Hash(current_map) = current {
                        if current_map.contains_key(&terminus) {
                            return Ok(());
                        }
                    }
                    path = format!("{}{}{}", path, path_separator, key);
                    path_separator = String::from(".") ;
                    continue ;
                }
                
                for candidate in current_components {
                    writer.write_fmt(format_args!("{}{}\n", path, candidate))? ;
                }
                return Ok(()) ;
            }
            Yaml::Array(arr) => {
                if !captures.get(INDEX_MATCH).is_some() {
                    return Err(Error::new(std::io::ErrorKind::Other, "illegal input no index for array")); // nothing to be done
                }
                let index = captures.get(INDEX_MATCH).unwrap().as_str().parse::<usize>().unwrap();
                path = format!("{}[{}]", path, index);
                current = &arr[index];
                continue;
            }
            _ => { return Ok(()); } // at this point anything else is a scalar or may as well be
        }

    } // for

    /*
     * dump current
     */
    match current {
        Yaml::Hash(map) => {
            for (key, _) in map {
                writer.write_fmt(format_args!("{}{}{}\n", path, path_separator, key.as_str().unwrap()))?;
                
            }
        }
        Yaml::Array(arr) => {
            for index in 0..arr.len() {
                writer.write_fmt(format_args!("{}[{}]\n", path, index))?;
            }
        }
        _ => { writer.write_fmt(format_args!("{}\n", path))? ;  }
    }

    Ok(())
}