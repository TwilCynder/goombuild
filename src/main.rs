use std::{fmt::Display, process::exit};

use config::Config;
use read_yaml::{get_hash, read_yaml_file};

mod config;
mod read_yaml;

fn handle_read_error(err: &dyn Display) -> ! {
    println!("Can't read config file : {err}");
    exit(1);
}

fn main() {
    let filename = "./gbuild.yaml";

    let docs = read_yaml_file(&filename).unwrap_or_else(|err| handle_read_error(&err));
    let data = get_hash(&docs).unwrap_or_else(|err| handle_read_error(&err));

    let config = Config::read(data).unwrap_or_else(|err| {
        println!("Incorrect config content : {err}");
        exit(2)
    });
    config.write("./Makefile");
}
