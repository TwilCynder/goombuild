use crate::read_yaml::{array_or_string_into_vec, get_array, get_bool, get_data, get_int, get_str, handle_wrong_type, try_map_option, ContentError, ContextfulError, ContextfulMaybe, YamlResult};

use super::{BuildConfig, Config, SourceDir, Target};
use yaml_rust2::{yaml::Hash, Yaml};



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

    fn read_from_hash(&mut self, hash: &'a Hash) -> Result<(), ContextfulError>{
        if let Some(str) = get_str(&hash, "dir")?{
            self.dir = str;
        } else {
            return Err(ContextfulError::from("Source directory is missing").add_context("In source directory"))
        }
        self.ext = get_str(&hash, "ext")?;
        self.depth = get_int(&hash, "depth")?;

        if let Some(yaml) = get_data(hash, "exclude") {self.exclude = array_or_string_into_vec(yaml)?}        
        if let Some(b) = get_bool(&hash, "included")? {self.included = b};
        Ok(())
    }

    fn read_from_hash_or_string(yaml: &'a Yaml) -> Result<SourceDir<'a>, ContextfulError> {
        let mut source = Self::new();
        match yaml {
            Yaml::String(str) => Ok(source.read_from_str(str)),
            Yaml::Hash(hash) => source.read_from_hash(hash),
            val => Err(handle_wrong_type(val, "string or object")).into()
        }?;
        Ok(source)
    }
}

impl <'a> BuildConfig<'a> {
    fn read(&mut self, data: &'a Hash) -> Result<(), ContextfulError>{
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
            None => return Err(ContentError::Other("Targets need a name"))
        };

        let mut target = Target::new(name);

        target.config.read(data).add_context(|| "in target ".to_owned() + name)?;

        Ok(target)
    }
}

impl <'a> Config<'a> {
    pub fn read(data: &'a Yaml) -> Result<Config<'a>, ContentError> {
        let mut config = Config::new();

        match &data {
            Yaml::Hash(hash) => {

                if let Some(str) = get_str(hash, "kind")? {
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

                if let Some(yaml) = get_data(hash, "sources") {
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
                    if let Some(str) = get_str(hash, "src_ext")?{config.default_ext = str};
                    if let Some(yaml) = get_data(hash, "src_exclude"){config.exclude_dir = array_or_string_into_vec(yaml)?}
                } else {
                    let Some(source) = config.source.get_mut(0) else {unreachable!("The source vector should never be empty (index 0 should always be valid)")};
                    if let Some(str) = get_dir_name(hash, "src_dir")? {source.dir = str};
                    if let Some(yaml) = get_data(hash, "src_exclude"){
                        source.exclude = array_or_string_into_vec(yaml)?
                    }
                    source.depth = get_int(hash, "src_depth")?;
                    source.ext = get_str(hash, "src_ext")?;

                }
                if let Some(yaml) = get_data(hash, "include") {
                    config.include_dir = array_or_string_into_vec(yaml)?;
                }
                for source in &config.source {
                    if source.included {
                        config.include_dir.push(source.dir);
                    }
                }
                if let Some(b) = get_bool(hash, "keep_source_dir_names")? {config.keep_source_dir_names = b};
                if let Some(str) = get_dir_name(hash, "obj_dir")? {config.obj_dir = str};
                if let Some(str) = get_dir_name(hash, "bin_dir")? {config.bin_dir = str};

                config.default_config.read(hash).add_context(|| "In default config")?;
                if let Some(array) = get_array(hash, "targets")? {
                    for data in array {
                        match data {
                            Yaml::Hash(hash) => {
                                config.alt_targets.push(Target::read(hash)?);
                            },
                            val => return Err(handle_wrong_type(val, "table").add_context("targets property").into())
                        }
                    }
                }

                config.output_file = get_str(hash, "output-file")?.or(get_str(hash, "output_file")?);

                return Ok(config);
            },
            val => Err(handle_wrong_type(val, "property list").add_context("Config file").into())
        }
    }
}