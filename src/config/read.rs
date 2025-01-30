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

fn to_unsinged(source: i64) -> Result<u64, ContentError> {
    match source.try_into(){
        Err(_) => Err(ContentError::WrongType("should be a positive integer")),
        Ok(n) => Ok(n),
    }
}

fn handle_wrong_type(yaml_value: &Yaml, expected: &'static str) -> ContentError {
    match yaml_value {
        Yaml::BadValue => ContentError::from("Invalid yaml"),
        _ =>ContentError::wrong_type(expected)
    }
}

fn get_data<'a>(data: &'a Hash, key: &'static str) -> Option<&'a Yaml> {
    data.get(&Yaml::from_str(key))
}

type YamlResult<T> = Result<Option<T>, ContentError>;

fn get_as<'a, T, F: Fn(&'a Yaml) -> YamlResult<T>>(extract: F, data: &'a Hash, key: &'static str) -> YamlResult<T> {
    match get_data(data, key){
        None => Ok(None),
        Some(v) => extract(v)
    }
}

fn extract_str(yaml: &Yaml) -> YamlResult<&str> {
    match yaml {
        Yaml::String(str) => Ok(Some(str.as_str())),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
fn get_str<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a str>{get_as(extract_str, data, key)}


fn extract_int(yaml: & Yaml) -> YamlResult<i64> {
    match yaml {
        Yaml::Integer(n) => Ok(Some(*n)),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
fn get_int(data: &Hash, key: &'static str) -> YamlResult<i64>{get_as(extract_int, data, key)}

fn extract_hash<'a>(yaml: &'a Yaml) -> Result<Option<&'a Hash>, ContentError> {
    match yaml {
        Yaml::Hash(hash) => Ok(Some(hash)),
        val => Err(handle_wrong_type(val, "table"))
    }
}
fn get_hash<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a Hash>{get_as(extract_hash, data, key)}

fn array_or_string_into_vec<'a>(yaml: &'a Yaml, vec: &mut Vec<&'a str>) -> Result<(), ContentError> {
    vec.clear();
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
    Ok(())
}

impl <'a> Config<'a> {
    pub fn read(data: &'a Yaml) -> Result<Config<'a>, ContentError> {
        let mut config = Config::new();

        match &data {
            Yaml::Hash(data) => {
                if let Some(str) = get_str(data, "exec")? {config.exec_name = str};

                if let Some(yaml) = get_data(data, "include_dir") {
                    array_or_string_into_vec(yaml, &mut config.include_dir)?
                }
                if let Some(str) = get_str(data, "src_dir")? {config.src_dir = str};
                if let Some(hash) = get_hash(data, "sources")? {
                    if let Some(str) = get_str(hash, "dir")? {config.src_dir = str}
                    if let Some(n) = get_int(hash, "depth")? {config.src_depth = to_unsinged(n)?}
                }
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