use serde_derive::Deserialize;
use std::fs;
use std::vec::Vec;

#[derive(Deserialize)]
pub struct Config {
    pub package: Package,
    pub bind: Bind,
    pub etc: Etcetera,
}


#[derive(Deserialize)]
pub struct Bind {
    pub read: ReadOnly,
    pub dev: Dev,
    pub tmpfs: Tmpfs,
    pub ext: Extra,
}


#[derive(Deserialize)]
pub struct ReadOnly {
    pub dirs: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Dev {
    pub dirs: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Tmpfs {
    pub dirs: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Extra {
    pub proc: Option<Vec<String>>,
    pub dev: Option<Vec<String>>,


}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub bin: String,
    pub args: Option<Vec<String>>,

}

#[derive(Deserialize)]
pub struct Etcetera {
    pub die_with_parent: Option<bool>,
}


pub fn parse_file(path: &str) -> Config{
    println!("Path: {}", path);
    let x: Config = toml::from_str(&fs::read_to_string(path).expect("Failed to read path.")).expect("Failed to parse file");
    return x;
}



