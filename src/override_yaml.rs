
use std::fmt::Debug;

use yaml_rust2::Yaml;

use crate::read_yaml::{get_data_mut, yaml_type_name, ContextfulError, ContextfulMaybe};

#[derive(Default, gumdrop::Options)]
pub struct Overrides {
    overrides: Vec<String>
}

impl Overrides {
    pub fn add(&mut self, ov: String){
        self.overrides.push(ov)
    }

    pub fn apply(&self, data: &mut Yaml) -> Result<(), ContextfulError> {
        for ov in &self.overrides {
            let split = ov.split_once("=");
            match split {
                None => return Err(ContextfulError::from("Config override syntax is `path=value`").add_context(format!("In "))),
                Some((path, value)) => {
                    override_property(data, path, value).add_context(||format!("In property override {ov}"))?;
                }
            }
        };
        Ok(())
    }
}

impl Debug for Overrides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Overrides").field("overrides", &self.overrides).finish()
    }
}



fn create_value(value: &str) -> Yaml {
    yaml_rust2::Yaml::from_str(value)
}

/*
fn traverse_path_existing<'a, 'b>(mut data: &'a mut Yaml, path: &'b[&str]) -> Result<(&'a mut Yaml, &'b str, &'b str), ContextfulError>{
    let iter = path.iter().enumerate();
    let (_, mut name) = if let Some(s) = iter.next() {s} else {return Err(ContextfulError::from("Empty path"))};

    while let Some((i, name)) = iter.next() {
        match name.chars().nth(0) {
            None => return Err(ContextfulError::from("Empty name in the property path")),
            /*Some(c) => if c.is_digit(10){
                match data {
                    Yaml::Array(arr) => {
                        let i: usize = name.parse().map_err(|err|ContextfulError::from(format!("Couldn't parse list index {name} : not a valid number")))?;
                        match arr.get(i) {
                            Some (val) => val,
                            None => {
                                return Err(ContextfulError::from(format!("List index {name} is out of bounds")))
                            }
                        };

                    },
                    _ => return Err(ContextfulError::from(format!("Trying to index list element in non-list value (is a {}", yaml_type_name(&data))))
                }
            }*/
            Some('+') => {
                return 
            }
        }
    };
}
    */

fn construct_from_path(path_fields: &[&str], value: &str) -> Result<Yaml, ContextfulError>{
    let mut value = create_value(value);
    for name in path_fields.iter().rev() {
        match name.chars().nth(0) {
            None => return Err(ContextfulError::from("Empty name in the property path")),
            Some('+') => {
                let mut new_value = Yaml::Array(Vec::new());
                let Yaml::Array(arr) = &mut new_value else {unreachable!("Yaml::from_str with empty litteral [] returned wrong type")};
                arr.push(value);
                value = new_value;
            },
            Some(c) if c.is_digit(10) => return Err(ContextfulError::from(format!("Cannot index element {name} in empty array"))),
            Some(_) => {
                let mut new_value = Yaml::Hash(yaml_rust2::yaml::Hash::new());
                //println!("{new_value:?}");
                let Yaml::Hash(hash) = &mut new_value else {unreachable!()};
                hash.insert(Yaml::from_str(name), value);
                value = new_value;
            }

        }
    }

    Ok(value)
}
 
fn traverse_path(data: &mut Yaml, path: &[&str], value: &str) -> Result<(), ContextfulError> {
    if let Some(name) = path.first() {
        match name.chars().nth(0) {
            None => return Err(ContextfulError::from("Empty name in the property path")),
            Some('+') => {
                match data {
                    Yaml::Array(arr) => {
                        arr.push(construct_from_path(&path[1..], value)?);
                        return Ok(())
                    },
                    _ => return Err(ContextfulError::from(format!("Trying to append element to non-list value (is a {})", yaml_type_name(&data))))
                }
            },
            Some(c) if c.is_digit(10) => {
                match data {
                    Yaml::Array(arr) => {
                        let i: usize = name.parse().map_err(|err: std::num::ParseIntError|ContextfulError::from(format!("Couldn't parse list index {name} : {err}")))?;
                        
                        if path.len() > 1 {
                            match arr.get_mut(i) {
                                Some (val) => traverse_path(val,&path[1..], value),
                                None => {
                                    arr[i] = construct_from_path(&path[1..], value)?;
                                    Ok(())
                                }
                            }
                        } else {
                            arr[i] = create_value(value);
                            Ok(())
                        }

                        

                    },
                    _ => return Err(ContextfulError::from(format!("Trying to index list element in non-list value (is a {}", yaml_type_name(&data))))
                }
            },
            Some(_) => {
                match data {
                    Yaml::Hash(hash) => {
                        if path.len() > 1 {
                            match get_data_mut(hash, name) {
                                None => {
                                    hash.insert(Yaml::from_str(name), construct_from_path(&path[1..], value)?);
                                    Ok(())
                                },
                                Some(yaml) => traverse_path(yaml, &path[1..], value)

                            }
                        } else {
                            hash.insert(Yaml::from_str(name), create_value(value));
                            Ok(())
                        }
                        
                    }
                    _ => return Err(ContextfulError::from(format!("Trying to index property in non-hash value (is a {}", yaml_type_name(&data))))
                }
            }
        }
    } else {
        unreachable!("traverse_path called with empty range")
    }
}

pub fn override_property(data: &mut Yaml, path: &str, value: &str) -> Result<(), ContextfulError> {
    let split: Vec<&str> = path.split('.').collect();

    traverse_path(data, &split.as_slice(), value)
}

/*
pub fn override_property(data: &mut Yaml, path: &str, value: &str) -> Result<(), ContextfulError>{
    let mut current_yaml_value = data;
    let split: Vec<&str> = path.split('.').collect();
    for name in &split.as_slice()[..split.len() - 1] {
        current_yaml_value = match name.chars().nth(0) {
            None => return Err(ContextfulError::from("Empty name in the property path")),
            Some('+') => {
                match current_yaml_value {
                    Yaml::Array(arr) => {
                        arr.push(construct_from_path(&split.as_slice(), value)?);
                        return Ok(())
                    },
                    _ => return Err(ContextfulError::from(format!("Trying to append element to non-list value (is a {}", yaml_type_name(&current_yaml_value))))

                }
            },
            Some(c) if c.is_digit(10) => {
                match current_yaml_value {
                    Yaml::Array(arr) => {
                        let i: usize = name.parse().map_err(|err: std::num::ParseIntError|ContextfulError::from(format!("Couldn't parse list index {name} : {err}")))?;
                        match arr.get_mut(i) {
                            Some (val) => val,
                            None => {
                                return Err(ContextfulError::from(format!("List index {name} is out of bounds")))
                            }
                        }

                    },
                    _ => return Err(ContextfulError::from(format!("Trying to index list element in non-list value (is a {}", yaml_type_name(&current_yaml_value))))
                }
            }, 
            Some(_) => {
                match current_yaml_value {
                    Yaml::Hash(hash) => {

                        if let Some(val) = get_data_mut(hash, &name) {
                            val
                        } else {
                            unsafe {
                                let ptr = &mut * hash;
                            }
                            hash.insert(Yaml::from_str(name), construct_from_path(&split.as_slice(), value)?);
                            return Ok(())
                        }

                        /*
                        match get_data_mut(hash, &name) {
                            None => {
                                    hash.insert(Yaml::from_str(name), construct_from_path(&split.as_slice(), value)?);
                                return Ok(());
                            },
                            Some (val) => val,
                        }
                        */

                    },
                    _ => return Err(ContextfulError::from(format!("Trying to index property in non-hash value (is a {}", yaml_type_name(&current_yaml_value))))
                }
            }
        };
    };



    Ok(())
}
*/