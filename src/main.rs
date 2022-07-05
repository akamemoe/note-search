use std::{env, fs};
use std::error::Error;
use std::path::Path;
use std::ffi::OsStr;


use walkdir::WalkDir;

pub struct Config{
    pub extensions :Vec<String>
}

impl Config {
    pub fn new(extensions :&[String]) -> Option<Config>{
        OK(Config{extensions:extensions})
    }
    pub fn eligible(self:&Self, path :&str,metadata :&fs::Metadata) -> bool{
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        let ext = Path::new(path).extension().and_then(OsStr::to_str)?;
        if last_modified < 24 * 3600 && self.extensions.contains(&ext){
            println!("eligible: {}",path);
            return true;
        }
        return false;
    }
}

fn add_document(path :&str){
    println!("adding: {}",path);
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(vec![".txt",".md",".yml",".yaml",".ini",".properties",".py",".c",".cpp",".go",".java"])
    let current_dir = env::current_dir()?;
    let args: Vec<String> = env::args().collect();
    let paths = Vec<String>::new();
    if args.len() > 1{
        paths.append(args[1..]);
    }else{
        paths.push(current_dir);
    }
    for path in paths {
        let metadata = fs::metadata(&path)?;
        if metadata.is_dir(){
            for entry in WalkDir::new(&path) {
                let entry = entry.unwrap();
                println!("walking {}", entry.path().display());
                let metadata = fs::metadata(&path)?;
                if metadata.is_file() && config::eligible(&path) {
                    println!("found file: {}",path);
                    add_document(&path);
                }
            }
        }
        if metadata.is_file() && config::eligible(&path){
            add_document(&path);
        }
    }

    Ok(())
}