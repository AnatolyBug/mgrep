//! Mgrep
//! mgrep is a command to search a stirng in a file

#[cfg(test)]
mod tests;

//Box<dyn Error> means the funciton will return a Type that implements the Error trait
//smth like Err(Trait)
pub fn run(conf: Config2) -> Result<(), Box<dyn Error>>{
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
///Case sensitive search query in a Contents string
///
/// # Examples
/// ```
/// let query = "hello";
/// let contents = "\
/// hello Anatoly
/// How are you?";
/// assert_eq!(vec!["hello Anatoly"], mgrep::search(query, contents));
/// ```
pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    //let mut results: Vec<&str> = Vec::new(); //not good if we want concurrent access
    /*
    for line in contents.lines() {
        if line.contains(query){
            results.push(line)
        }
    }
    results */
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
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

pub struct Config2 {
    pub q: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config2 {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        //will be mutating args by iterating over it
        //since returning Err(&str) compiler needs to know the lifetime of str
        args.next();
        let q: String = match args.next() {
            Some(a) => a,
            None => return Err("Didn't get a query string")
        };
        let filename: String = match args.next() {
            Some(a) => a,
            None => return Err("Didn't get a filename")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //returns Err if not set
        Ok(Self { q, filename, case_sensitive}) //simplified
    }
}