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
    exec_name: &'a str, 
    include_dir: Vec<&'a str>,
    obj_dir: &'a str,
    keep_source_dir_names: bool,
    source: Vec<SourceDir<'a>>,
    default_ext: &'a str,
    compiler: &'a str,
    cflags: &'a str,
    ldflags: &'a str,
    libs: Vec<&'a str>
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            exec_name: "main",
            include_dir: Vec::from(["include"]),
            obj_dir: "obj",
            source: Vec::from([SourceDir::new()]),
            keep_source_dir_names: false,
            default_ext: "cpp",
            compiler: "g++",
            cflags: "",
            ldflags: "",
            libs: Vec::new()
        }    
    }
}

mod write;
mod read;