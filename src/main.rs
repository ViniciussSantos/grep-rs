use clap::{Arg, Command};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let cmd = Command::new("grep-rs")
        .about("A stripped down version of grep")
        .args([
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true)
                .index(1),
            Arg::new("input")
                .help("File to search")
                .required(true)
                .index(2),
        ])
        .get_matches();

    let pattern = cmd.get_one::<String>("pattern").unwrap();
    let re = regex::Regex::new(&pattern).unwrap();

    let default = "-".to_string();

    let input = cmd.get_one::<String>("input").unwrap_or(&default);

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let lines = line_.unwrap();
        if re.is_match(&lines) {
            println!("{}", lines);
        }
    }
}
