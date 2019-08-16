use hello_cargo::data_type::number::number_operation;
use std::{env, process};
use hello_cargo::Config;

fn main() {
    number_operation();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("config error :{}", err);
        process::exit(1);
    });
    if let Err(e) = hello_cargo::run(config) {
        eprintln!("Application  error :{}", e);
        process::exit(1);
    };
}