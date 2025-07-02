use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use yaml_rust::{YamlLoader};
use ucompleter::write_completions;

fn find_config_file(arg0: &str, env_var: &str) -> String {
    let home = env::var("HOME").unwrap_or("".to_string());
    let default_path = format!(".:{}/.config/ucompleter", home);
    let path = env::var(env_var).unwrap_or(default_path);
    let paths: Vec<&str> = path.split(':').collect();
    let target = format!("{}.yaml", arg0) ;
    
    for path in paths {
        let file_path = format!("{}/{}", path, target);
        if std::path::Path::new(&file_path).exists() {
            return file_path;
        }
    }
    panic!("no config file not found for {}", arg0);
}


fn main() {
    let argv: Vec<String> = env::args().collect();
    
    let config_path = find_config_file(&argv[1], "UCOMPLETER_PATH");


    let mut contents = String::new();
    let file = File::open(config_path).expect("Unable to open the file");
    let mut buf_reader = BufReader::new(file) ;
    buf_reader.read_to_string(&mut contents).expect("Unable to read the file");

    let current = &YamlLoader::load_from_str(&contents).expect("load/parse failed")[0];
    
    write_completions::write_completions(&mut std::io::stdout(), current, &argv[2]).expect("TODO: panic message");
    
}
