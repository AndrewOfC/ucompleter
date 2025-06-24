use std::io::{Error, Write};
use yaml_rust::Yaml;
use regex::Regex;

pub fn write_completions<W: Write>(writer: &mut W, inputtree: &Yaml, inputpath: &str) -> std::io::Result<()>
{
    let mut current = inputtree;
    let re = Regex::new(r"([^\.\[\]]+)(?:(?:\[(\d+)\])|(\[$))?").unwrap();
    let mut path = String::from("") ;
    let mut path_separator = String::from(""); // initially empty for root path

    for matches in re.captures_iter(inputpath) {
        if matches.get(3).is_some() {
            if let Yaml::Array(_) = current {
                return Err(Error::new(std::io::ErrorKind::Other, "illegal input"));
            }
            break;  
        }

        match current {
            Yaml::Hash(map) => {
                let m1 = matches.get(1).unwrap();
                let key = &inputpath[m1.start() .. m1.end()] ;
                let ykey = Yaml::String(key.to_string());
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
                    writer.write_fmt(format_args!("{}\n", candidate))? ;
                }
                return Ok(()) ;
            }
            Yaml::Array(arr) => {
                if !matches.get(2).is_some() {
                    return Err(Error::new(std::io::ErrorKind::Other, "illegal input no index for array")); // nothing to be done
                }
                let index = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
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
        _ => { return Ok(()); }
    }

    Ok(())
}