//! This module provides metadata retrieval and writing functionalities for processing YAML documents.
//!
//! The main functions in this module are:
//!
//! - `get_metadata`
//! - `write_completions`

use crate::get_metadata::get_metadata;
use crate::{keys_starting_with, yaml_descent};
use std::io::Write;
use regex::Regex;
use yaml_rust::Yaml;


const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const PERIOD_MATCH: usize = 2 ;
const INDEX_MATCH: usize = 3 ;
const ARRAY_MATCH: usize = 4;
// 
// pub fn make_ystring_vec(s : &str) -> Vec<&Yaml> {
//     vec![&Yaml::String(s.to_string())] 
// }

fn sep(y: &Yaml, empty_path: bool) -> &str {
    if empty_path { return "" ; }
    match y {
        Yaml::Hash(_) => { "." }
        Yaml::Array(_) => { "" }
        _ => { "" }
    }
}



pub fn write_completions<W: Write>(writer: &mut W, doc: &Yaml, ipath: &str, add_descriptions: &bool) -> std::io::Result<()>
{
    let mut path = ipath ;
    let re = Regex::new(r"([^.\[\]\\]+)(\.)?|(?:\\?\[(\d+)\\?\]?|(\\?\[$))?").unwrap() ;
    let metadata = get_metadata(&doc, *add_descriptions) ;
    let mut current = if !metadata.has_root() { doc } else { &doc.as_hash().unwrap()[&metadata.root] } ;
    let mut current_path = String::from("") ;
    let mut empty_path = true ;
    let mut match_iter = re.captures_iter(path).peekable();
    while let Some(captures) = match_iter.next() {
        let last = match_iter.peek().is_none();
        let mut key = captures.get(KEY_MATCH).map_or("", |m| m.as_str()) ;
        let mut index = captures.get(INDEX_MATCH).map_or(0, |m| m.as_str().parse::<usize>().unwrap()) ;
        let mut terminated = captures.get(PERIOD_MATCH).is_some() ;
        
        while true {
            match current {
                Yaml::Hash(hash) => {
                    let ykey = Yaml::String(key.to_string()) ;
                    // let key_vector = hash.iter().map(|(k,v)|k.as_str().unwrap() ).collect::<Vec<&str>>();
                    
                    if terminated {
                        // no need to search for members starting with key
                        if !hash.contains_key(&ykey) { return Ok(()) ; }
                        current = &hash[&ykey]; // next
                        
                        current_path += ykey.as_str().unwrap() ;
                        empty_path = false ;
                        current_path += sep(current, empty_path) ; 
                        if !last { break ;}
                        path = "" ;
                        key = "" ;
                        terminated = false ;
                        continue ;
                    }
                    
                    let keys = keys_starting_with(&key, hash, &metadata.ignore_fields) ;
                    if keys.is_empty() { return Ok(()) ; }
                    
                    if keys.len() == 1 {
                        let ykey = keys[0] ;
                        if hash.contains_key(ykey) {
                             current = &hash[ykey];
                             key = ykey.as_str().unwrap() ;
                             current_path += ykey.as_str().unwrap() ;
                             empty_path = false ;
                             if !metadata.has_terminal_field(current) {
                                 current_path += sep(current, empty_path);
                             }
                             if !last { break ;}
                             key = "" ;
                             path = "" ;
                             continue ;
                        }
                    }
                    
                    if hash.len() == 1 {
                        let key = hash.keys().next().unwrap();
                        current = &hash[key];
                        current_path += key.as_str().unwrap() ;
                        empty_path = false ;
                        current_path += sep(current, empty_path) ;
                        continue ;
                    }
                    
                    let sep = sep(current, empty_path) ;
                    for key in keys.iter().map(|k| k.as_str().unwrap()) {
                        writer.write_fmt(format_args!("{}{}\n", current_path, key))?;
                    }
                    return Ok(()) ;
                }
                Yaml::Array(array) => {
                    if captures.get(INDEX_MATCH).is_some() {
                        if index >= array.len() {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Index out of bounds")) ;
                        }
                        current = &array[index];
                        current_path += format!("[{}]", index).as_str() ;
                        if !metadata.has_terminal_field(current) {
                            current_path += sep(current, empty_path);
                        }
                        break ;
                    }
                    if array.len() == 1 {
                        current_path += "[0]" ;
                        empty_path = false ;
                        current = &array[0];
                        if !metadata.has_terminal_field(current) {
                            current_path += sep(current, empty_path);
                        }
                        break ;
                    }
                    for (index, _) in array.iter().enumerate() {
                        let mut path2 = current_path.to_string();
                        
                        if path2.ends_with("[") {
                            path2.truncate(path.len() - 1);
                        }
                        let index_str = format!("[{}]", index);
                        writer.write_fmt(format_args!("{}{}\n", path2, index_str))?;
                    }
                    return Ok(()) ;
                }
                _ => { break ;}

            }
        } // while true
    } // for captures
    writer.write_fmt(format_args!("{}\n", current_path))?;
    Ok(())
}

