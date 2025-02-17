use std::fmt::Display;

use yaml_rust2::yaml::{Array, Hash, Yaml};

use super::{BuildConfig, Config, SourceDir, Target};

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

fn _to_unsinged(source: i64) -> Result<u64, ContentError> {
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

fn try_map_option <T, U, F: Fn(T) -> Result<U, ContentError>> (opt: Option<T>, f: F) -> YamlResult<U> {
    Ok(match opt {
        None => None,
        Some(v) => Some(f(v)?)
    })
}

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


fn extract_bool(yaml: & Yaml) -> YamlResult<bool> {
    match yaml {
        Yaml::Boolean(b) => Ok(Some(*b)),
        _ => Err(handle_wrong_type(&yaml, "string"))
    }
}
fn get_bool(data: &Hash, key: &'static str) -> YamlResult<bool>{get_as(extract_bool, data, key)}

fn _extract_hash<'a>(yaml: &'a Yaml) -> Result<Option<&'a Hash>, ContentError> {
    match yaml {
        Yaml::Hash(hash) => Ok(Some(hash)),
        val => Err(handle_wrong_type(val, "table"))
    }
}
fn _get_hash<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a Hash>{get_as(_extract_hash, data, key)}

fn extract_array<'a>(yaml: &'a Yaml) -> Result<Option<&'a Array>, ContentError> {
    match yaml {
        Yaml::Array(array) => Ok(Some(array)),
        val => Err(handle_wrong_type(&val, "array"))
    }
}
fn get_array<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a Array>{get_as(extract_array, data, key)}

fn array_or_string_into_vec<'a>(yaml: &'a Yaml) -> Result<Vec<&'a str>, ContentError> {
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

fn get_dir_name<'a>(data: &'a Hash, key: &'static str) -> YamlResult<&'a str> {
    match get_str(data, key)?{
        Some(str) => if str.is_empty() {Ok(Some("."))} else {Ok(Some(str))},
        None => Ok(None),
    }
}

impl <'a> SourceDir <'a> {
    fn read_from_str(&mut self, str: &'a str){
        self.dir = str;
    }

    fn read_from_hash(&mut self, hash: &'a Hash) -> Result<(), ContentError>{
        if let Some(str) = get_str(&hash, "dir")?{
            self.dir = str;
        } else {
            return Err(ContentError::Other("Source directory is missing"))
        }
        self.ext = get_str(&hash, "ext")?;
        self.depth = get_int(&hash, "depth")?;
        if let Some(b) = get_bool(&hash, "included")? {self.included = b};
        Ok(())
    }

    fn read_from_hash_or_string(yaml: &'a Yaml) -> Result<SourceDir<'a>, ContentError> {
        let mut source = Self::new();
        match yaml {
            Yaml::String(str) => Ok(source.read_from_str(str)),
            Yaml::Hash(hash) => source.read_from_hash(hash),
            val => Err(handle_wrong_type(val, "string or object"))
        }?;
        Ok(source)
    }
}

impl <'a> BuildConfig<'a> {
    pub fn read(&mut self, data: &'a Hash) -> Result<(), ContentError>{
        self.exec_name = get_str(data, "exec")?;
        self.libs = try_map_option(get_data(data, "libs"), array_or_string_into_vec)?;
        self.ldflags = get_str(data, "link_options")?.or(get_str(data, "ldflags")?);
        self.cflags = get_str(data, "compile_options")?.or(get_str(data, "cflags")?);
        self.compiler = get_str(data, "compiler")?;

        Ok(())
    }
}

impl <'a> Target<'a> {
    pub fn read(data: &'a Hash) -> Result<Target, ContentError> {
        let name = match get_str(data, "name")? {
            Some(str) => str,
            None => return Err(ContentError::Other("Target needs a name"))
        };

        let mut target = Target::new(name);

        target.config.read(data)?;

        Ok(target)
    }
}

impl <'a> Config<'a> {
    pub fn read(data: &'a Yaml) -> Result<Config<'a>, ContentError> {
        let mut config = Config::new();

        match &data {
            Yaml::Hash(data) => {

                if let Some(str) = get_str(data, "kind")? {
                    match str {
                        "cpp" => {
                            config.default_config.compiler = Some("g++");
                            config.default_ext = "cpp"
                        },
                        "c" => {
                            config.default_config.compiler = Some("gcc");
                            config.default_ext = "c";
                        }
                        _ => return Err(ContentError::from("Incorrect kind : must be either c or cpp"))
                    }
                } 

                if let Some(yaml) = get_data(data, "sources") {
                    config.source.clear();
                    match yaml {
                        Yaml::Array(arr) => {
                            for yaml in arr {
                                config.source.push(SourceDir::read_from_hash_or_string(yaml)?);
                            }
                        },
                        val => {
                            config.source.push(SourceDir::read_from_hash_or_string(val)?)
                        }
                    }
                    if let Some(str) = get_str(data, "src_ext")?{config.default_ext = str};
                } else {
                    let Some(source) = config.source.get_mut(0) else {unreachable!("The source vector should never be empty (index 0 should always be valid)")};
                    if let Some(str) = get_dir_name(data, "src_dir")? {source.dir = str};
                    if let Some(n) = get_int(data, "src_depth")? {source.depth = Some(n)};
                    if let Some(str) = get_str(data, "src_ext")?{source.ext = Some(str)};
                }
                if let Some(yaml) = get_data(data, "include") {
                    config.include_dir = array_or_string_into_vec(yaml)?;
                }
                for source in &config.source {
                    if source.included {
                        config.include_dir.push(source.dir);
                    }
                }
                if let Some(b) = get_bool(data, "keep_source_dir_names")? {config.keep_source_dir_names = b};
                if let Some(str) = get_dir_name(data, "obj_dir")? {config.obj_dir = str};

                config.default_config.read(data)?;
                if let Some(array) = get_array(data, "targets")? {
                    for data in array {
                        match data {
                            Yaml::Hash(hash) => {
                                config.alt_targets.push(Target::read(hash)?);
                            },
                            val => return Err(handle_wrong_type(val, "table"))
                        }
                    }
                }

                return Ok(config);
            },
            val => Err(handle_wrong_type(val, "property list"))
        }
    }
}