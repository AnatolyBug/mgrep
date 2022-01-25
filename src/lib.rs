use std::{fs};
use std::error::Error;

//Box<dyn Error> means the funciton will return a Type that implements the Error trait
//smth like Err(Trait)
pub fn run(conf: Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(conf.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

impl<'a> Config<'a> {
    pub fn new(args: &'a[String]) -> Result<Self, &str> {
        if args.len() < 3 { return Err("Not Enough Arguments")}
        let q: &String = &args[1];
        let filename: &String = &args[2];
        Ok(Config { q, filename }) //simplified
    }
}

pub struct Config<'a> {
    pub q: &'a str,
    pub filename: &'a str
}