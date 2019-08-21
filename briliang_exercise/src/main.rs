use hello_cargo::data_type::number::number_operation;
use std::{env, process};
use hello_cargo::Config;

fn main() {
    number_operation();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = hello_cargo::run(config) {
        eprintln!("Application  error :{}", e);
        process::exit(1);
    };
}