//cargo run { i)creates exec in target/debug ii)executes }
//cargo build - builds ( produces exec )
//cargo check - builds without producing exec (fast)
//cargo build --release {produces exe in target/release }
use std::{env, process};
use mgrep::Config;

fn main() {
    //if you want invalid unicode use std::env::args_os
    //collect turns iterator into a vector
    let args: Vec<String> = env::args().collect();
    //let (q, filename) = parse_config(&args);
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    println!("Query: {}", conf.q);
    println!("Filename: {}", conf.filename);
    //Need to handle the case where run could return an error
    if let Err(e) = mgrep::run(conf){
        println!("Application error: {}", e);
        process::exit(1);
    };
}
