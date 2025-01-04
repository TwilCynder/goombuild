pub struct Config <'a> {
    exec_name: &'a str, 
    include_dir: &'a str,
    obj_dir: &'a str,
    src_dir: &'a str,
    src_ext: &'a str,
    compiler: &'a str
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            exec_name: "main",
            include_dir: "include",
            obj_dir: "obj",
            src_dir: "src",
            src_ext: "cpp",
            compiler: "g++",
        }    
    }
}

mod write;
mod read;