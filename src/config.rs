pub struct Config <'a> {
    exec_name: &'a str, 
    include_dir: Vec<&'a str>,
    obj_dir: &'a str,
    src_dir: &'a str,
    src_depth: u64,
    src_ext: &'a str,
    compiler: &'a str,
    cflags: &'a str
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            exec_name: "main",
            include_dir: Vec::from(["include"]),
            obj_dir: "obj",
            src_dir: "src",
            src_depth: 0,
            src_ext: "cpp",
            compiler: "g++",
            cflags: ""
        }    
    }
}

mod write;
mod read;