mod json_object;
mod json_parser;
mod json_tokenizer;

use json_parser::Parser as JsonParser;

use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap_or_default();
    let file_contents = fs::read_to_string(file_path)?;

    let json_parser: JsonParser = JsonParser::new();

    match json_parser.parse(file_contents.as_str()) {
        Ok(_) => {
            println!("File is valid JSON");
            Ok(())
        }
        Err(error) => {
            println!("{}", error);
            Err(Box::new(error))
        }
    }
}
