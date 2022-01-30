//cargo run { i)creates exec in target/debug ii)executes }
//cargo build - builds ( produces exec ) dev profile
//cargo build --release #release profile
//cargo check - builds without producing exec (fast)
//cargo build --release {produces exe in target/release }
//cargo run > output.txt

use std::{env, process};
use mgrep::{Config2};

fn main() {
    /* Old version
    //if you want invalid unicode use std::env::args_os
    //collect turns iterator into a vector
    let args: Vec<String> = env::args().collect();
    //let (q, filename) = parse_config(&args);
    let conf = Config::new(&args).unwrap_or_else(|err| {
        //write to standard error
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    }); */
    //passing the ownership of the iterator
    let conf = Config2::new(env::args()).unwrap_or_else(|err| {
        //write to standard error
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    //write to standard output
    println!("Query: {}", conf.q);
    println!("Filename: {}", conf.filename);
    //Need to handle the case where run could return an error
    if let Err(e) = mgrep::run(conf){
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
