mod solution1;
use solution1::process_input;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let contents = read_file("input.txt").unwrap();
    match process_input(&contents) {
        Some(v) => println!("The answer is: {}", v),
        None => println!("value not found"),
    }
}
