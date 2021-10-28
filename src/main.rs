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
use colored::Colorize;

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

    let mut parser = MonkeyCParser::new(tokens, file_path.clone());
    match parser.parse() {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(errors) => {
            for error in errors {
                println!("{} {}", "|".bright_red(), error.full_msg.as_str().bright_red());

                // Getting a line specified inside of error struct
                let file = File::open(file_path.clone()).unwrap();
                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    if index + 1 == error.at.0 as usize {
                        print!("{}", "| ".cyan());
                        println!("{}", String::from(line.unwrap()).cyan());
                        break;
                    }
                }

                print!("{}", "| ".cyan());

                // Adding an underline
                for _ in 0..error.at.1 - 1 {
                    print!(" ");
                }
                for _ in 0..error.literal_len {
                    print!("{}", "~".cyan())
                }
                println!();

                print!("{}", "| ".cyan());

                // Adding a small pointer
                for _ in 0..error.at.1 - 1 {
                    print!(" ");
                }
                println!("{}", "| ".cyan());

                print!("{}", "| ".cyan());

                for _ in 0..error.at.1 - 1 {
                    print!(" ");
                }
                println!("{}\n", "| here".cyan());
            }
        }
    }

    Ok(())
}

