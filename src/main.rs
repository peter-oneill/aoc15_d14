use std::{fs,process};

struct Reindeer {
    name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Failed to read input.txt.  Error:\n{}", err);
        process::exit(1);
    });

    println!("{}", input_text);
    let speeds = parse_input(input_text).unwrap_or_else(|err| {
        eprintln!("Couldn't parse contents of input.txt.  Error:\n{}", err);
        process::exit(2);
    });
}

fn parse_input(input: String) -> Result<Vec<Reindeer>, &'static str> {
    if false {
        Ok(Vec::new())
    } else {
        Err("bad things!")
    }
}
