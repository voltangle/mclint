mod lexer;
mod parser;

use crate::lexer::MonkeyCLexer;
use anyhow::Context;
use anyhow::Result;
use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;
use crate::parser::MonkeyCParser;

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
    let tokens = lexer.lex().with_context(|| format!("Failed to tokenize {:?}", file_path))?;

    let mut parser = MonkeyCParser::new(tokens);
    parser.parse().with_context(|| format!("Failed to parse {:?}", file_path))?;

    println!("{:?}", matches.value_of("INPUT"));
    Ok(())
}
