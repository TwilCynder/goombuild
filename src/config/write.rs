use std::{collections::HashSet, fs::File, io::{self, Write}};

use super::{init_default, Config, Target};

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

fn concat_str<'a, T: ToString>(str1: &'static str, str2: T) -> String {
    str1.to_owned() + (&str2.to_string())
}

fn concat_str_post<'a, T: ToString>(str1: T, str2: &'static str) -> String {
    (str1.to_string()) + &str2.to_owned()
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
    pub fn write(&self, filename: &str){
        match self.write_(filename) {
            Err(msg) => println!("Couldn't write Makefile at path {filename} : {msg}"),
            _ => println!("Successfully wrote config to {filename}")
        }
    }

    fn write_(&self, filename: &str) -> Result<(), io::Error>{
        let mut file = File::create(filename)?;

        let defaults = init_default();

        macro_rules! or_default {
            ($config: expr, $prop_name: ident) => {
                $config.$prop_name.as_ref().unwrap_or(&defaults.$prop_name)
            };
        }

        //--- Variables
        write_var(&mut file, b"CC", or_default!(self.default_config, compiler))?;
        write_var(&mut file, b"EXEC", or_default!(self.default_config, exec_name))?;
        //write_var(&mut file, b"INCLUDE_DIR", self.include_dir)?;
        //write_var(&mut file, b"SRC_DIR", self.src_dir)?;
        write_var(&mut file, b"OBJ_DIR", self.obj_dir)?;
        write_var(&mut file, b"BIN_DIR", self.obj_dir)?;

        //write_var(&mut file, b"SRC_EXT", self.src_ext)?;
        write_var(&mut file, b"CFLAGS", or_default!(self.default_config, cflags))?;
        write_var(&mut file, b"LDFLAGS", or_default!(self.default_config, ldflags))?;

        //--- Libs
        let libs = or_default!(self.default_config, libs);
        file.write(b"LIBS=")?;
        for lib in libs {
            file.write(b"-l")?;
            file.write(lib.as_bytes())?;
            file.write(b" ")?;
        }
        nl(&mut file)?;

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
            let ext = if let Some(ext) = source.ext {ext} else {self.default_ext};
            let dir = source.dir;

            write!(file, "
_SRC= $(shell find {dir}{} -name \"*.{ext}\")
_OBJS= $(_SRC:.{ext}=.o)
OBJS := $(OBJS) $(patsubst {dir}/%,$(OBJ_DIR)/{}%,$(_OBJS))
                ", 
                string_if_option(source.depth, |depth: i64| string_if(depth > 0, || concat_str(" -maxdepth ", depth))),
                string_if(self.keep_source_dir_names, || concat_str_post(dir, "/"))
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

$(BIN_DIR)/$(EXEC): $(OBJS)
\t$(CC) $^ -o $@ $(LDFLAGS) $(LIBS)

clear: 
\t-@rm -f $(BIN_DIR)/$(EXEC) 2> /dev/null
\t-@rm -fr $(OBJ_DIR)/* 2> /dev/null

        ")?;

        if self.keep_source_dir_names {
            let mut exts = HashSet::<String>::new();
            for source in &self.source {
                let ext = match source.ext {
                    Some(str) => str,
                    None => self.default_ext
                };
                
                if !exts.contains(ext) {
                    exts.insert(ext.to_owned());
                    write!(file, "
$(OBJ_DIR)/%.o: %.{ext}
\t@mkdir -p $(dir $@)
\t$(CC) $(INCLUDE) -c $< -o $@
                        ",
                    )?;
                }
            }
        } else {
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
        }

        for target in &self.alt_targets {
            target.write(&mut file)?;
        }

        Ok(())
    }
}

fn write_target_name(file: &mut File, name: &str) -> Result<(), std::io::Error>{
    file.write(name.as_bytes())?;
    file.write(b": ")?;
    Ok(())
}

fn write_target_var(file: &mut File, varname: &[u8], val: Option<&str>, target_name: &str) -> Result<(), io::Error> {
    if let Some(val) = val {
        write_target_name(file, target_name)?;
        write_var(file, varname, val)?;
    };
    Ok(())
}

impl<'a> Target<'a> {
    pub fn write(&self, file: &mut File) -> Result<(), std::io::Error>{
        nl(file)?;
        let mut dependency = "all";
        if let Some(exec_name) = self.config.exec_name {
            write!(file,"
$(BIN_DIR)/{}: $(OBJS)
\t$(CC) $^ -o $@ $(LDFLAGS) $(LIBS)
            ", exec_name)?;
            dependency = exec_name;
        }

        write_target_var(file, b"CC", self.config.compiler, self.name)?;
        write_target_var(file, b"CFLAGS", self.config.cflags, self.name)?;
        write_target_var(file, b"LDFLAGS", self.config.ldflags, self.name)?;

        if let Some(libs) = &self.config.libs {
            write_target_name(file, self.name)?;
            file.write(b"LIBS=")?;
            for lib in libs {
                file.write(b"-l")?;
                file.write(lib.as_bytes())?;
                file.write(b" ")?;
            }
        }

        write!(file, "
{}: {dependency}
        ", self.name)?;

        Ok(())
    }
}