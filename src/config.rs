pub struct Config <'a> {
    exec_name: &'a str, 
    obj_dir: &'a str,
    src_dir: &'a str
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { 
            exec_name: "main",
            obj_dir: "obj",
            src_dir: "src"
        }    
    }
}

mod write;
mod read;