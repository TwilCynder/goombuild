use std::{error::Error, fmt::Display, fs, process::exit};

use read_yaml::{get_hash, read_yaml_file, ReadError};
use yaml_rust2::{yaml::Hash, Yaml, YamlLoader};
use config::Config;

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
}
