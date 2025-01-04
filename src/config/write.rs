use std::{fs::File, io::{self, Write}};

use super::Config;

impl Config <'_>{
    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::create(filename)?;

        file.write(b"EXEC=")?;
        file.write(self.exec_name.as_bytes())?;
        file.write(b"\n")?;

        file.write(b"SRC_DIR=")?;
        file.write(self.src_dir.as_bytes())?;
        file.write(b"\n")?;

        file.write(b"OBJ_DIR=")?;
        file.write(self.obj_dir.as_bytes())?;
        file.write(b"\n")?;

        Ok(())
    }

    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path {filename} : {msg}"),
            _ => println!("Successfully wrote config to {filename}")
        }
    }
}