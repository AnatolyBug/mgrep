use std::{fs, env};
use std::error::Error;

#[cfg(test)]
mod tests;

//Box<dyn Error> means the funciton will return a Type that implements the Error trait
//smth like Err(Trait)
pub fn run(conf: Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(conf.filename)?;

    let res = if conf.case_sensitive {
        search(&conf.q, &contents)
    } else {
        search_case_insensitive(&conf.q, &contents)
    };

    for line in  res {
        println!("{}", line)
    }
    Ok(())
}

impl<'a> Config<'a> {
    pub fn new(args: &'a[String]) -> Result<Self, &str> {
        if args.len() < 3 { return Err("Not Enough Arguments")}
        let q: &String = &args[1];
        let filename: &String = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //returns Err if not set
        Ok(Config { q, filename, case_sensitive}) //simplified
    }
}

pub struct Config<'a> {
    pub q: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    //let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()){
            results.push(line)
        }
    }
    results
}