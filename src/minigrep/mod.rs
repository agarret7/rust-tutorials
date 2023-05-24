use std::{fs, error::Error};

pub struct Config {
    pub query : String,
    pub file_path : String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// "dyn" is short for "dynamic"
// "Box<dyn Error>" is a "trait object", this means the function
// will return a type that implements the "Error" trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // recall "?"" returns either a Result or Option.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}