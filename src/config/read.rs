use std::fmt::Display;

use yaml_rust2::yaml::{Hash, Yaml};

use super::Config;

#[derive(Debug)]
pub enum ContentError {
    Other(&'static str),
    WrongType(&'static str)
}

impl From<&'static str> for ContentError {
    fn from(error: &'static str) -> Self {
        ContentError::Other (error)
    }
}

impl ContentError {
    fn wrong_type(expected: &'static str) -> Self {
        ContentError::WrongType(expected)
    }
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ContentError::Other(error) => f.write_str(&error),
            ContentError::WrongType(expected) => {
                f.write_str("Should be a ")?;
                f.write_str(&expected)
            }
        }
    }
}

fn handle_wrong_type(yaml_value: &Yaml, expected: &'static str) -> ContentError {
    match yaml_value {
        Yaml::BadValue => ContentError::from("Invalid yaml"),
        _ =>ContentError::wrong_type(expected)
    }
}

impl Config<'_> {
    fn read_exec_name(&mut self, data: &Hash) -> Result<(), ContentError>{
        match data.get(&Yaml::from_str("exec_name")) {
            None => Ok(self.exec_name = "main"),
            Some(v) => match v {
                Yaml::String(_) => todo!(),
                val => Err(handle_wrong_type(val, "string"))
            }
        }
    }

    pub fn read(data: &Yaml) -> Result<Config, ContentError> {
        let mut config = Config::new();

        match &data {
            Yaml::Hash(hash) => {
                config.read_exec_name(hash)?;
                return Ok(config);
            },
            val => Err(handle_wrong_type(val, "property list"))
        }
    }
}