use std::{
    fs,
    io::{self, BufRead, BufReader},
    path,
};

mod token;

fn main() {
    run_file("input/hello_world.lox");
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(source) => {
            token::tokenize(source);
        }
        Err(err) => println!("{:?}", err),
    }
}
