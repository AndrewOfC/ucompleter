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
    use clap::{Arg, Command};

    /*
    println!("Command line arguments:");
    for (index, argument) in std::env::args().enumerate() {
        eprintln!("  [{}]: {}", index, argument);
    }*/

    let options = Command::new("ucompleter")
        .arg(Arg::new("command")
            .help("Command to complete")
            .required(true))
        .arg(Arg::new("prefix")
            .help("Completion prefix")
            .default_value(""))
        .arg(Arg::new("wordbefore")
            .help("bash word before last entry")
            .required(false))
        .arg(Arg::new("descriptions")
            .short('d')
            .long("descriptions")
            .help("output descriptions if available")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let command = options.get_one::<String>("command").unwrap();
    let config_path = find_config_file(command, format!("{}_PATH", command.to_uppercase()).as_str());


    let mut contents = String::new();
    let file = File::open(config_path).expect("Unable to open the file");
    let mut buf_reader = BufReader::new(file) ;
    buf_reader.read_to_string(&mut contents).expect("Unable to read the file");

    let current = &YamlLoader::load_from_str(&contents).expect("load/parse failed")[0];

    let prefix = options.get_one::<String>("prefix").unwrap();

    write_completions::write_completions(&mut std::io::stdout(),
                                         current,
                                         prefix,
                                         options.get_one::<bool>("descriptions").unwrap()).expect("TODO: panic message");
}
