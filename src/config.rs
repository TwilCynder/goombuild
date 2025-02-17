struct SourceDir<'a> {
    dir: &'a str,
    ext: Option<&'a str>,
    depth: Option<i64>,
    included: bool
}

impl <'a> SourceDir<'a> {
    pub fn new() -> SourceDir <'a> {
        SourceDir {
            dir: "src",
            ext: None,
            depth: None,
            included: false
        }
    }
}

pub struct Config <'a> {
    keep_source_dir_names: bool,
    source: Vec<SourceDir<'a>>,
    default_ext: &'a str,
    include_dir: Vec<&'a str>,
    obj_dir: &'a str,

    default_config: BuildConfig<'a>,
    alt_targets: Vec<Target<'a>>
}

#[derive(Debug)]
pub struct Target<'a> {
    name: &'a str,
    config: BuildConfig<'a>
}

impl <'a> Target<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            config: BuildConfig::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct BuildConfig<'a> {
    exec_name: Option<&'a str>,
    compiler: Option<&'a str>,
    cflags: Option<&'a str>,
    ldflags: Option<&'a str>,
    libs: Option<Vec<&'a str>>
}

pub struct DefaultConfig {
    exec_name: &'static str, 
    compiler: &'static str,
    cflags: &'static str,
    ldflags: &'static str,
    libs: Vec<&'static str>
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            source: Vec::from([SourceDir::new()]),
            keep_source_dir_names: false,
            default_ext: "cpp",
            include_dir: Vec::new(),
            obj_dir: "obj",


            default_config: BuildConfig::default(),
            alt_targets: Vec::new()
        }    
    }
}

fn init_default() -> DefaultConfig {
    DefaultConfig {
        exec_name: "main",
        compiler: "gcc",
        cflags: "",
        ldflags: "",
        libs: Vec::new()
    }
}

mod write;
mod read;