mod solutions;
use anyhow::Result;
use clap::load_yaml;
use clap::App;
use solutions::process_input;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    // command line arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_file = matches.value_of("input").unwrap_or("input.txt");

    // read input file, process the input
    let contents = read_file(input_file)?;
    match process_input(&contents) {
        Some(v) => println!("The answer is: {}", v),
        None => println!("value not found"),
    }
    Ok(())
}
