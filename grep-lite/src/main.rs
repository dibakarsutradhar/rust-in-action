use regex::Regex;
use clap::{App, Arg};

fn main() {
    // Incrementally builds a command argument parser, where each argument takes an Arg.
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
             .help("The pattern to search for")
             .takes_value(true)
             .required(true))
        .get_matches();

    // Extracts the pattern argument
    let pattern = args.value_of("pattern").unwrap();

    // unwrap() unwraps a Result, crashing if an error occurs.
    let re = Regex::new("picture").unwrap();

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
}
