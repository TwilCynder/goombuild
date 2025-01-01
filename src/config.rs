pub struct Config <'a> {
    exec_name: &'a str, 
}

impl<'a> Config <'a>{
    pub fn new() -> Config <'a> {
        Config { exec_name: "main" }    
    }
}

mod write;
mod read;