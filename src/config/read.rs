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

fn get_str <'a>(data: &'a Hash, key: &'static str) -> Result<Option<&'a str>, ContentError>{
    match data.get(&Yaml::from_str(key)) {
        None => Ok(None),
        Some(v) => match v {
            Yaml::String(str) => Ok(Some(&str.as_str())),
            val => Err(handle_wrong_type(val, "string"))
        }
    }
}

impl <'a> Config<'a> {

    pub fn read(data: &Yaml) -> Result<Config, ContentError> {
        let mut config = Config::new();

        match &data {
            Yaml::Hash(data) => {
                if let Some(str) = get_str(data, "exec")? {config.exec_name = str};
                if let Some(str) = get_str(data, "include_dir")? {config.include_dir = str};
                if let Some(str) = get_str(data, "src_dir")? {config.src_dir = str};
                if let Some(str) = get_str(data, "obj_dir")? {config.obj_dir = str};
                if let Some(str) = get_str(data, "kind")? {
                    match str {
                        "cpp" => {
                            config.compiler = "g++";
                            config.src_ext = "cpp"
                        },
                        "c" => {
                            config.compiler = "gcc";
                            config.src_ext = "c";
                        }
                        _ => return Err(ContentError::from("Incorrect kind : must be either c or cpp"))
                    }
                }   
                if let Some(str) = get_str(data, "src_ext")? {config.src_ext = str};
                if let Some(str) = get_str(data, "compiler")? {config.compiler = str};
                if let Some(str) = get_str(data, "compile_flags")? {config.cflags = str};


                return Ok(config);
            },
            val => Err(handle_wrong_type(val, "property list"))
        }
    }
}