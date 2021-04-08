use std::fs;


mod parse;
mod load;


const PATHDIR: &str = "/home/theo/dev/rpkg/toml";

fn main(){
    
    let x = load::return_all(PATHDIR);

    for conf in x {
        println!("{}", conf.package.name);
    } 

    

}
