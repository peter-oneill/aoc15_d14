use std::{fs,process};

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Failed to read input.txt.  Err: \"{}\"", err);
        process::exit(1);
    });
    println!("{}", input_text);
}
