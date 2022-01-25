use super::*;

#[test]
fn case_sensitive() {
    let q = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
DucTree.";
    assert_eq!(vec!["safe, fast, productive."], search(q, contents))
}

#[test]
fn case_insensitive() {
    let q = "rUSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
    assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(q, contents))
}