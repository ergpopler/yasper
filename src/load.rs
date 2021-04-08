// This file will find and load all toml files in PATHDIR


use std::fs;
use crate::parse::Config;
use crate::parse;
use std::vec::Vec;

pub fn return_all(pathdir: &str) -> Vec<Config> {
    
    let paths = std::fs::read_dir(pathdir).expect("Failed to read directory");    
    
    
    let mut configs = Vec::new();
    
    for path in paths {
        let p = &format!("{}", path.unwrap().path().display());
        if p.ends_with(".toml"){
            configs.push(parse::parse_file(p));
        } else {
            println!("It seems there is a non TOML file in {}, this shouldn't be the case, this will be ignored though!", &pathdir);
        }
        
    }
    return configs


}
