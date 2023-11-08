use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    // Incrementally builds a command argument parser, where each argument takes an Arg.
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
             .help("The pattern to search for")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("input")
             .help("File to search")
             .takes_value(true)
             .required(true))
        .get_matches();

    // Extracts the pattern argument
    let pattern = args.value_of("pattern").unwrap();

    // unwrap() unwraps a Result, crashing if an error occurs.
    let re = Regex::new(pattern).unwrap();
    
    let input = args.value_of("pattern").unwrap_or("-");

    let quote = "Every face, every shop, bedroom window, public-house, and 
    dark quare is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    // Implements a contains() method that searches for a substring.
    for line in quote.lines() {
        let contains_substring = re.find(line);

        match contains_substring {
            // Some(T) is the positive case of an Option, meaning that re.find() was successful.
            // It matches all values
            Some(_) => println!("{}", line),
           
            // None is the negative case of an Option. () can be thought of as a null placeholder
            // value here.
            None => {},
        }
    }

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
    
}
