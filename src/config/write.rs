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

fn string_if_option<T, F: Fn(T)->String>(opt: Option<T>, expr: F) -> String{
    match opt {
        Some(val) => expr(val),
        None => "".to_owned()
    }
}

impl Config <'_>{
    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::create(filename)?;

        //--- Variables
        write_var(&mut file, b"CC", self.compiler)?;
        write_var(&mut file, b"EXEC", self.exec_name)?;
        //write_var(&mut file, b"INCLUDE_DIR", self.include_dir)?;
        //write_var(&mut file, b"SRC_DIR", self.src_dir)?;
        write_var(&mut file, b"OBJ_DIR", self.obj_dir)?;
        //write_var(&mut file, b"SRC_EXT", self.src_ext)?;
        write_var(&mut file, b"CFLAGS", self.cflags)?;

        //--- Include path
        file.write(b"INCLUDE=")?;
        for dir in &self.include_dir {
            file.write(b"-I")?;
            file.write(dir.as_bytes())?;
            file.write(b" ")?;
        }
        nl(&mut file)?;

        nl(&mut file)?;

        //--- Processing sources into objs
        file.write(b"OBJS=")?;
        for source in &self.source {
            println!("{} {:?} {:?}", source.dir, source.ext, source.depth);

            write!(file, "
_SRC= $(shell find {}{} -name \"*.{}\")
_OBJS= $(_SRC:.$(SRC_EXT)=.o)
OBJS += $(patsubst $(SRC_DIR)/%,$(OBJ_DIR)/%,$(_OBJS))
                ", 
                source.dir,
                string_if_option(source.depth, |depth: i64| string_if(depth > 0, || concat_str(" -maxdepth ", depth))),
                if let Some(ext) = source.ext {ext} else {self.default_ext}
            )?;

        }
        nl(&mut file)?;

        //--- Vpath
        for source in &self.source {
            file.write(b"vpath %.")?;
            file.write((if let Some(ext) = source.ext {ext} else {self.default_ext}).as_bytes())?;
            file.write(b" ")?;
            file.write(source.dir.as_bytes())?;
        }
        nl(&mut file)?;

        //--- Static boilerplate (Phony, all, start/mkdir, clear)
        write!(file, "
.PHONY: start

all: start $(EXEC)

start:
	mkdir -p $(OBJ_DIR)

$(EXEC): $(OBJS)
\t$(CC) $^ -o $@ $(LDFLAGS) 

clear: 
\t-@rm -f $(EXEC) 2> /dev/null
\t-@rm -fr $(OBJ_DIR)/* 2> /dev/null

        ")?;

        for source in &self.source {
            write!(file, "
$(OBJ_DIR)/%.o: {}/%.{}
\t@mkdir -p $(dir $@)
\t$(CC) $(INCLUDE) -c $< -o $@
                ",
                source.dir,
                if let Some(ext) = source.ext {ext} else {self.default_ext}
            )?;
        }

        Ok(())
    }

    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path {filename} : {msg}"),
            _ => println!("Successfully wrote config to {filename}")
        }
    }
}