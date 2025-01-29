use std::{fs::File, io::{self, Write}};

use super::Config;

fn nl(file: &mut File) -> Result<(), io::Error> {
    file.write(b"\n")?;
    Ok(())
}

fn writeln(file: &mut File, str: &str) -> Result<(), io::Error>{
    file.write(str.as_bytes())?;
    nl(file)?;
    Ok(())
}

fn write_var(file: &mut File, varname: &[u8], val: &str) -> Result<(), io::Error> {
    file.write(varname)?;
    file.write(b"=")?;
    writeln(file, val)?;
    Ok(())
}

fn concat_str<T: ToString>(str1: &'static str, str2: T) -> String {
    str1.to_owned() + (&str2.to_string())
}

fn string_if<F: Fn()->String>(cond: bool, expr: F) -> String{
    if cond {
        expr()
    } else {
        "".to_owned()
    }
}

impl Config <'_>{
    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::create(filename)?;

        write_var(&mut file, b"CC", self.compiler)?;
        write_var(&mut file, b"EXEC", self.exec_name)?;

        file.write(b"INCLUDE=")?;
        for dir in &self.include_dir {
            file.write(b"-I")?;
            file.write(dir.as_bytes())?;
            file.write(b" ")?;
        }
        nl(&mut file)?;

        //write_var(&mut file, b"INCLUDE_DIR", self.include_dir)?;
        write_var(&mut file, b"SRC_DIR", self.src_dir)?;
        write_var(&mut file, b"OBJ_DIR", self.obj_dir)?;
        write_var(&mut file, b"SRC_EXT", self.src_ext)?;
        write_var(&mut file, b"CFLAGS", self.cflags)?;

        nl(&mut file)?;

        file.write(format!("
SRC= $(shell find $(SRC_DIR) {} -name \"*.$(SRC_EXT)\")
_OBJS= $(SRC:.$(SRC_EXT)=.o)
OBJS = $(patsubst $(SRC_DIR)/%,$(OBJ_DIR)/%,$(_OBJS))

vpath %.h $(INCLUDE_DIR)
vpath %.$(SRC_EXT) $(SRC_DIR)

.PHONY: start

all: start $(EXEC)

start:
	mkdir -p $(OBJ_DIR)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.$(SRC_EXT)
\t@mkdir -p $(dir $@)
\t$(CC) -I$(INCLUDE) -c $< -o $@

$(EXEC): $(OBJS)
\t$(CC) $^ -o $@ $(LDFLAGS) 

clear: 
\t-@rm $(EXEC)
\t-@rm -r $(OBJ_DIR)/*

        ", 
        string_if(self.src_depth > 0, || concat_str("-maxdepth ", self.src_depth))).as_bytes()
    )?;

        Ok(())
    }

    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path {filename} : {msg}"),
            _ => println!("Successfully wrote config to {filename}")
        }
    }
}