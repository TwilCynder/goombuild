use std::{fmt::Display, fs, process::exit};

use config::Config;
use gumdrop::Options;
use read_yaml::{get_doc, read_yaml_file};

mod config;
mod read_yaml;
mod options;
mod override_yaml;

fn handle_read_error(err: &dyn Display) -> ! {
    println!("Can't read config file : {err}");
    exit(1);
}

const INPUT_FILENAMES : [&str ; 4] = [
    "./gbuild.yaml",
    "./gbuild.yml",
    "./goombuild.yaml",
    "./goombuild.yml"
];

fn find_input_file(input_filename: &Option<String>) -> &str{
    match input_filename {
        Some(str) => {
            match fs::exists(str){
                Ok(true) => str,
                other => {
                    match other {
                        Err(err) => println!("Couldn't find input file :  {err}"),
                        Ok(_) => println!("Couldn't find input file")
                    }
                    exit(1);
                }
            }
        },
        None => {
            let mut found_filename = None;
            for filename_ in INPUT_FILENAMES {
                match fs::exists(filename_) {
                    Ok(true) => found_filename = Some(filename_),
                    _ => ()
                }
            }
            found_filename.unwrap_or_else(|| {
                print!("Can't open config file : looked for any of ");
                for path in &INPUT_FILENAMES[..INPUT_FILENAMES.len() - 1] {print!("{path}, ")}
                println!("or {}", match &INPUT_FILENAMES.last(){Some(str)=>str,None=>unreachable!()});
                exit(1);
            })
        }
    }
    
}

fn main() {
    let options = options::Options::parse_args_default_or_exit();

    let filename = find_input_file(&options.input_file);

    let docs = read_yaml_file(&filename).unwrap_or_else(|err| handle_read_error(&err));
    let data = get_doc(&docs).unwrap_or_else(|err| handle_read_error(&err));

    let config = Config::read(data).unwrap_or_else(|err| {
        println!("Incorrect config content : {err}");
        exit(2)
    });
    config.write(options.out_file.as_ref().map_or(config.output_file.unwrap_or("./Makefile"), String::as_str));
}
