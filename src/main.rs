mod lexer;
mod parser;
#[cfg(test)]
mod tests;

use crate::lexer::MonkeyCLexer;
use anyhow::Context;
use anyhow::Result;
use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;
use crate::parser::MonkeyCParser;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::slice::range;

fn main() -> Result<()> {
    let matches = App::new("MCLint")
        .version("0.1.0")
        .author("GGorAA <yegor_yakovenko@icloud.com>")
        .about("A linter for Monkey C language.")
        .arg(
            Arg::with_name("INPUT")
                .help("An input file to lint")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_path = PathBuf::from(matches.value_of("INPUT").unwrap());
    let file_contents = fs::read_to_string(file_path.clone())
        .with_context(|| format!("Failed to read contents of {}", file_path.display()))?;

    let mut lexer = MonkeyCLexer::new(file_contents.chars().collect());
    let tokens = lexer.lex().with_context(|| format!("Failed to tokenize {:?}", file_path.clone()))?;

    let mut parser = MonkeyCParser::new(tokens);
    match parser.parse() {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(errors) => {
            for error in errors {
                let mut error_str = String::new();

                // Getting a line specified inside of error struct
                let file = File::open(file_path.clone()).unwrap();
                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    if index + 1 == error.at.0 as usize {
                        error_str.push_str(&*line.unwrap());
                        break;
                    }
                }
                error_str.push('\n');

                // Adding an underline
                for _ in 0..error.at.1 {
                    error_str.push(' ');
                }
                for _ in 0..error.literal_len {
                    error_str.push('~');
                }
                error_str.push('\n');

                // Adding a small pointer
                for _ in 0..error.at.1 {
                    error_str.push(' ');
                }
                error_str.push_str("|\n");

                for _ in 0..error.at.1 {
                    error_str.push(' ');
                }
            }
        }
    }

    println!("{:?}", matches.value_of("INPUT"));
    Ok(())
}

