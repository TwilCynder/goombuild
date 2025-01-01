use std::fs;

use yaml_rust2::{yaml::Hash, Yaml, YamlLoader};
use config::Config;

mod config;
mod errors;

fn handle_config(data: &Yaml){
    
}

fn main() {
    let raw_input = fs::read_to_string("./test/test.yaml").expect("Couldn't read file");
    let doc = YamlLoader::load_from_str(&raw_input);

    match &doc {
        Err(msg) => println!("Couldn't open config file : {msg}"),
        Ok(v) => {
            for yaml in v {
                handle_config(yaml);
            }
        }
    }
}
