use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

fn main() {
    let config_path = "dumper.yaml" ;
    let mut contents = String::new();
    let file = File::open(config_path).expect("Unable to open the file");
    let mut buf_reader = BufReader::new(file) ;
    buf_reader.read_to_string(&mut contents).expect("Unable to read the file");
    let mut current = &YamlLoader::load_from_str(&contents).expect("load/parse failed")[0];
    let re = Regex::new(r"([^\.\[\]]+)|(?:(?:\[(\d+)\])|(\[$))?").unwrap();
    let argv: Vec<String> = env::args().collect();
    let path = &argv[1];

    for matches in re.captures_iter(path) {
        match current {
            Yaml::Hash(hash) => {
                if matches.get(1).is_none() {
                    eprintln!("{} badaccessor", path);
                    std::process::exit(2);
                }
                let m1 = matches.get(1).unwrap();
                let key = &path[m1.start() .. m1.end()] ;
                let ykey = Yaml::String(key.to_string());
                if !hash.contains_key(&ykey) {
                    eprintln!("{} not found", key);
                    std::process::exit(2);
                }
                current = &hash[&ykey];
                continue ;
            }
            Yaml::Array(array) => {
                let index = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
                if index >= array.len() {
                    eprintln!("{} out of range", index);
                    std::process::exit(2);
                }
                current = &array[index];
                continue ;
            }
            _ => {
                break ;
            }
        }

    }

    let mut outs = String::new() ;
    let mut emitter = YamlEmitter::new(&mut outs);
    emitter.dump(&current).unwrap();
    println!(
        "{}",
        outs
    ) ;


}