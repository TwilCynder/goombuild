use std::{fs::File, io::{self, Write}};

use super::Config;

fn writeln(file: &mut File, str: &str) -> Result<(), io::Error>{
    file.write(str.as_bytes())?;
    file.write(b"\n")?;
    Ok(())
}

fn write_var(file: &mut File, varname: &[u8], val: &str) -> Result<(), io::Error> {
    file.write(varname)?;
    file.write(b"=")?;
    writeln(file, val)?;
    Ok(())
}

impl Config <'_>{
    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::create(filename)?;

        write_var(&mut file, b"CC", self.compiler)?;
        write_var(&mut file, b"EXEC_NAME", self.exec_name)?;
        write_var(&mut file, b"INCLUDE_DIR", self.include_dir)?;
        write_var(&mut file, b"SRC_DIR", self.src_dir)?;
        write_var(&mut file, b"OBJ_DIR", self.obj_dir)?;
        write_var(&mut file, b"SRC_EXT", self.src_ext)?;
        write_var(&mut file, b"CFLAGS", self.cflags)?;

        Ok(())
    }

    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path {filename} : {msg}"),
            _ => println!("Successfully wrote config to {filename}")
        }
    }
}