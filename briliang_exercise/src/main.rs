use briliang_exercise::data_type::number::number_operation;
use std::{env, process};
use briliang_exercise::Config;

fn main() {
    number_operation();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = briliang_exercise::run(config) {
        eprintln!("Application  error :{}", e);
        process::exit(1);
    };
}