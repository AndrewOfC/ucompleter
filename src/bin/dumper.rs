use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read};
use yaml_rust::YamlLoader;

use ucompleter::write_completions::write_completions;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config_path = "dumper.yaml" ;
    let mut contents = String::new();
    let mut file = File::open(config_path).expect("Unable to open the file");
    let mut buf_reader = BufReader::new(file) ;
    buf_reader.read_to_string(&mut contents).expect("Unable to read the file");
    let mut path : String = String::from("");

    let mut current = &YamlLoader::load_from_str(&contents).expect("load/parse failed")[0];
    
    write_completions(&mut stdout(), &current, &argv[2]) ;
}