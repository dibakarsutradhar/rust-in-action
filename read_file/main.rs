use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // creates a File object that requires a path argument and error handling if the file does not
    // exits.
    // This program crashes if a readme.md is not present.
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    // reuses a single String object over the lifetime of the program
    let mut line = String::new();

    // because reading from disk can fall, we need to explicitly handle this.
    // in this case, errors crash the program
    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);

        // Shrinks the String back to length 0, preventing lines from persisting into the following
        // ones
        line.truncate(0);
    }
}
