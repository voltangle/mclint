use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new("MCLint")
        .version("0.1.0")
        .author("GGorAA <yegor_yakovenko@icloud.com>")
        .about("A linter for Monkey C language.")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to lint")
            .required(true)
            .index(1))
        .get_matches();
}
