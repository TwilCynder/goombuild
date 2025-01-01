use std::{fs::{self, File}, io::{self, Write}};

use super::Config;

impl Config <'_>{
    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::open(filename)?;

        file.write(b"EXEC=")?;
        file.write(self.exec_name.as_bytes())?;

        Ok(())
    }

    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path ${filename} : ${msg}"),
            _ => ()
        }
    }
}