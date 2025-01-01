use std::{error::Error, fs, io};

use yaml_rust2::{yaml::{self, Hash, Yaml}, ScanError, YamlLoader};

use super::Config;

enum ReadErrorTypes {
    IO(io::Error),
    YamlScan(ScanError),
    Other(&'static str)
}

pub struct ReadError {
    err: ReadErrorTypes,
    msg: &'static str
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError {err: ReadErrorTypes::IO(error), msg: "Could not open config file"}
    }
}

impl From<ScanError> for ReadError {
    fn from(error: ScanError) -> Self {
        ReadError {err: ReadErrorTypes::YamlScan(error), msg: "Could not open config file"}
    }
}

impl From<&'static str> for ReadError {
    fn from(error: &'static str) -> Self {
        ReadError {err: ReadErrorTypes::Other(error), msg: "Could not open config file"}
    }
}

impl Config<'_> {
    fn read_exec_name(&self, data: &Hash){

    }

    fn read_(data: &Yaml) -> Result<Config, ReadError> {
        let config = Config::new();

        match &data {
            Yaml::Hash(hash) => {
                config.read_exec_name(hash);
                return Ok(config);
            },
            Yaml::BadValue => Err(ReadError::from("Invalid yaml")),
            _ => Err(ReadError::from("Invalid yaml : should be a property list"))
        }
    }

    pub fn read(filename: &str) -> Result<(), ReadError> {
        let raw_input = fs::read_to_string("./test/test.yaml")?;
        let doc = YamlLoader::load_from_str(&raw_input)?;

        Ok(())
    }
}