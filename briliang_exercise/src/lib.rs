//! # Mini Grep
//! 1. First option is query str
//! 2. Second is file name
use std::{fs, env};
use std::error::Error;

pub mod data_type;
mod function;
mod control_flow;
mod ownership;
mod structure;
mod enums;
mod module;
mod collections;
mod error_handle;
mod generic_trait_lifetime;
mod functional_feature;
mod smart_pointer;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

//这个是trait object，dyn 是dynamic的意思
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

/// search query string in content inline
/// # Examples
/// ```
/// let arg = "dd";
/// let vec = briliang_exercise::search(arg,"ddaa");
/// assert_eq!(vec!["ddaa"],vec);
/// ```
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lower_case_query = query.to_lowercase();
    content.lines().filter(|line| line.to_lowercase().contains(&lower_case_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

