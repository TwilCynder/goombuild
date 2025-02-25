use std::{error::Error, fmt::Display, fs, io};

use yaml_rust2::{ScanError, Yaml, YamlLoader};
use yaml_rust2::yaml::{Array, Hash};


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

pub fn get_doc(docs: &Vec<Yaml>) -> Result<&Yaml, &str> {
    if docs.len() > 1{
        return Err("Config file somehow contains multiple YAML documents");
    }
    if docs.len() < 1{
        return Err("Empty config");
    }

    Ok(&docs[0])
}


pub enum ContextfulErrorType {
    WrongType(&'static str, &'static str),
    Other(&'static str)
}

pub struct ContextfulError {
    err: ContextfulErrorType,
    context: String
}

impl ContextfulError {
    pub fn _with_context<T: ToString>(err: ContextfulErrorType, context_str: impl Fn() -> T) -> Self {
        Self{err, context: context_str().to_string()}
    }

    pub fn wrong_type(expected: &'static str, got: &Yaml) -> Self {
        Self{err: ContextfulErrorType::WrongType(expected, yaml_type_name(&got)), context: String::new()}
    }

    pub fn add_context<T: ToString>(mut self, context: T) -> Self {
        self.context = context.to_string() + " " + self.context.as_str();
        self
    }
}

impl From<&'static str> for ContextfulError {
    fn from(error: &'static str) -> Self {
        ContextfulError { err: ContextfulErrorType::Other (error), context: String::new() }
    }
}

pub enum ContentError {
    Other(&'static str),
    Contextful(ContextfulError)
}

impl From<&'static str> for ContentError {
    fn from(error: &'static str) -> Self {
        ContentError::Other (error)
    }
}

impl From<ContextfulError> for ContentError {
    fn from(value: ContextfulError) -> Self {
        ContentError::Contextful(value)
    }
}

impl Display for ContextfulError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.context)?;
        match &self.err {
            ContextfulErrorType::WrongType(expected, got) => {
                write!(f, "should be a {expected} (got {got})")
            },
            ContextfulErrorType::Other(mdsg) => f.write_str(&mdsg),
        }
    }
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ContentError::Other(error) => f.write_str(&error),
            ContentError::Contextful(err )=> {
                err.fmt(f)
            }
        }
    }
}

fn yaml_type_name(yaml_value: &Yaml) -> &'static str {
    match yaml_value {
        Yaml::Real(_) => "real number",
        Yaml::Integer(_) => "integer",
        Yaml::String(_) => "string",
        Yaml::Boolean(_) => "boolean",
        Yaml::Array(_) => "list (aka array)",
        Yaml::Hash(_) => "table (aka hash)",
        Yaml::Alias(_) => "yaml alias",
        Yaml::Null => "null",
        Yaml::BadValue => "error value",
    }
}

fn _to_unsinged(source: i64) -> Result<u64, ContextfulError> {
    match source.try_into(){
        Err(_) => Err("should be a positive integer".into()),
        Ok(n) => Ok(n),
    }
}

pub fn handle_wrong_type(yaml_value: &Yaml, expected: &'static str) -> ContextfulError {
    match yaml_value {
        Yaml::BadValue => " : Invalid YAML value".into(),
        _ => ContextfulError::wrong_type(expected, yaml_value)
    }
}

pub fn get_data<'a>(data: &'a Hash, key: &'static str) -> Option<&'a Yaml> {
    data.get(&Yaml::from_str(key))
}

pub type YamlResult<T> = Result<Option<T>, ContextfulError>;

pub fn try_map_option <T, U, F: Fn(T) -> Result<U, ContextfulError>> (opt: Option<T>, f: F) -> YamlResult<U> {
    Ok(match opt {
        None => None,
        Some(v) => Some(f(v)?)
    })
}

fn get_as<'a, T, F: Fn(&'a Yaml) -> YamlResult<T>>(extract: F, data: &'a Hash, key: &'static str) -> YamlResult<T> {
    match get_data(data, key){
        None => Ok(None),
        Some(v) => extract(v).map_err(|err| err.add_context(key))
    }
}

fn extract_str(yaml: &Yaml) -> YamlResult<&str> {
    match yaml {
        Yaml::String(str) => Ok(Some(str.as_str())),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
pub fn get_str<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a str>{get_as(extract_str, data, key)}

fn extract_int(yaml: & Yaml) -> YamlResult<i64> {
    match yaml {
        Yaml::Integer(n) => Ok(Some(*n)),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
pub fn get_int(data: &Hash, key: &'static str) -> YamlResult<i64>{get_as(extract_int, data, key)}


fn extract_bool(yaml: & Yaml) -> YamlResult<bool> {
    match yaml {
        Yaml::Boolean(b) => Ok(Some(*b)),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
pub fn get_bool(data: &Hash, key: &'static str) -> YamlResult<bool>{get_as(extract_bool, data, key)}

fn _extract_hash<'a>(yaml: &'a Yaml) -> Result<Option<&'a Hash>, ContextfulError> {
    match yaml {
        Yaml::Hash(hash) => Ok(Some(hash)),
        val => Err(handle_wrong_type(val, "table"))
    }
}
fn _get_hash<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a Hash>{get_as(_extract_hash, data, key)}

fn extract_array<'a>(yaml: &'a Yaml) -> Result<Option<&'a Array>, ContextfulError> {
    match yaml {
        Yaml::Array(array) => Ok(Some(array)),
        val => Err(handle_wrong_type(&val, "array"))
    }
}
pub fn get_array<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a Array>{get_as(extract_array, data, key)}

pub fn array_or_string_into_vec<'a>(yaml: &'a Yaml) -> Result<Vec<&'a str>, ContextfulError> {
    let mut vec = Vec::<&'a str>::new();
    match yaml {
        Yaml::String(str) => {
            vec.push(str);
        },
        Yaml::Array(arr) => {
            for yaml in arr {
                if let Some(str) = extract_str(yaml)? {
                    vec.push(str);
                }
            };
        }
        val => return Err(handle_wrong_type(val, "string or array thereof"))
    };
    Ok(vec)
}