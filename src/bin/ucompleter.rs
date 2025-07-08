use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use yaml_rust::{YamlLoader};
use aep_rust_common::descender::Descender;
use aep_rust_common::find_config_file::find_config_file;
use aep_rust_common::yaml_descender::YamlDescender;


fn ucompleter_verbose() -> bool {
    match env::var("UCOMPLETER_VERBOSE") {
        Ok(value) => value.parse::<i32>().unwrap_or(0) > 0,
        Err(_) => false
    }
}

fn main() {
    use clap::{Arg, Command};
    
    /*
     * for debugging completions
     */
    #[cfg(debug_assertions)]
    {
        if ucompleter_verbose() {
            eprintln!("Command line arguments:");
            for (index, argument) in std::env::args().enumerate() {
                eprintln!("  [{}]: '{}'", index, argument);
            }
        }
    }

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
        .arg(Arg::new("zshellmode")
            .short('z')
            .long("zshmode")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let command = options.get_one::<String>("command").unwrap();
    let prefix = options.get_one::<String>("prefix").unwrap();
    let bash_or_zsh = !options.get_one::<bool>("zshellmode").unwrap();
    let wordbefore = match options.get_one::<String>("wordbefore") {
        Some(w) => w,
        None => ""
    } ;

    let config_path = match find_config_file(command, format!("{}_PATH", command.to_uppercase()).as_str()) {
      Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let descender = match YamlDescender::new_from_file(&config_path, bash_or_zsh) {
        Ok(descender) => descender,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } ;
    
    let mut contents = String::new();
    let file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    let mut buf_reader = BufReader::new(file) ;
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }


    let path = if prefix == "@" {
        wordbefore.to_owned() + prefix
    } else {
        prefix.to_owned()
    } ;
    
    descender.write_completions(&mut std::io::stdout(),
                                         &path,
                                         false/*wip*/).expect("TODO: panic message");
}
