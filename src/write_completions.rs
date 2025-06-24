use std::io::{Error, Write};
use yaml_rust::Yaml;
use regex::Regex;

const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ; 
const PERIOD_MATCH: usize = 2 ;
const INDEX_MATCH: usize = 3 ;
const ARRAY_MATCH: usize = 4;

pub fn write_completions<W: Write>(writer: &mut W, inputtree: &Yaml, inputpath: &str) -> std::io::Result<()>
{
    let mut current = inputtree;
    let re = Regex::new(r"([^\.\[\]]+)(\.)?|(?:(?:\[(\d+)\])|(\[$))?").unwrap();
    let mut path = String::from("") ;
    let mut path_separator = String::from(""); // initially empty for root path

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