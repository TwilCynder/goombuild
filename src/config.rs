struct SourceDir<'a> {
    dir: &'a str,
    ext: Option<&'a str>,
    depth: Option<i64>
}

impl <'a> SourceDir<'a> {
    pub fn new() -> SourceDir <'a> {
        SourceDir {
            dir: "src",
            ext: None,
            depth: None
        }
    }
}

pub struct Config <'a> {
    exec_name: &'a str, 
    include_dir: Vec<&'a str>,
    obj_dir: &'a str,
    source: Vec<SourceDir<'a>>,
    default_ext: &'a str,
    compiler: &'a str,
    cflags: &'a str
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            exec_name: "main",
            include_dir: Vec::from(["include"]),
            obj_dir: "obj",
            source: Vec::from([SourceDir::new()]),
            default_ext: "cpp",
            compiler: "g++",
            cflags: ""
        }    
    }
}

mod write;
mod read;