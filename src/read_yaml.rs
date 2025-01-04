use std::{error::Error, fmt::Display, fs, io};

use yaml_rust2::{ScanError, Yaml, YamlLoader};

#[derive(Debug)]
pub enum ReadError {
    IO(io::Error),
    YamlScan(ScanError),
    Content(&'static str)
}

impl Error for ReadError {
    
}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match &self {
            ReadError::IO(error) => {
                let str = error.to_string();
                f.write_str(&str)
            }
            ReadError::YamlScan(scan_error) => {
                let str = scan_error.to_string();
                f.write_str(&str)
            },
            ReadError::Content(str) => {
                f.write_str(str)
            }
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError::IO(error)
    }
}

impl From<ScanError> for ReadError {
    fn from(error: ScanError) -> Self {
        ReadError::YamlScan(error)
    }
}

impl From<&'static str> for ReadError {
    fn from(error: &'static str) -> Self {
        ReadError::Content(error)
    }
}

pub fn read_yaml_file(filename: &str) -> Result<Vec<Yaml>, ReadError> {
    let raw_input = fs::read_to_string(filename)?;
    let docs = YamlLoader::load_from_str(&raw_input)?;

    Ok(docs)
}

pub fn get_hash(docs: &Vec<Yaml>) -> Result<&Yaml, &str> {
    if docs.len() > 1{
        return Err("Config file somehow contains multiple YAML documents");
    }
    if docs.len() < 1{
        return Err("Empty config");
    }

    Ok(&docs[0])
}