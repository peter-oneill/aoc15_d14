use regex::Regex;
use std::{fs, process};

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32,
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Failed to read input.txt.  Error:\n{}", err);
        process::exit(1);
    });

    println!("{}", input_text);

    let reindeers = parse_input(input_text).unwrap_or_else(|err| {
        eprintln!("Couldn't parse contents of input.txt.  Error:\n{}", err);
        process::exit(2);
    });

    for reindeer in reindeers {
        println!("{:?}", reindeer);
    }
}

fn parse_input(input: String) -> Result<Vec<Reindeer>, String> {
    let mut reindeers: Vec<Reindeer> = Vec::new();

    let speed_regex = Regex::new(
        "(?P<name>[a-zA-Z]+) \
            can fly (?P<speed>\\d+) km/s \
            for (?P<flight_time>\\d+) seconds, \
            but then must rest for (?P<rest_time>\\d+) seconds.",
    )
    .unwrap();

    for line in input.lines() {
        let regex_match = speed_regex.captures(line);

        if let None = regex_match {
            return Err(format!("Couldn't match line: \"{}\"", line));
        }

        let regex_match = regex_match.unwrap();

        let name = regex_match.name("name").unwrap().as_str().to_owned();
        let speed: u32 = extract_u32_from_regex(&regex_match, "speed")?;
        let flight_time: u32 = extract_u32_from_regex(&regex_match, "flight_time")?;
        let rest_time: u32 = extract_u32_from_regex(&regex_match, "rest_time")?;

        reindeers.push(Reindeer {
            name,
            speed,
            flight_time,
            rest_time,
        });
    }

    Ok(reindeers)
}

fn extract_u32_from_regex(
    regex_match: &regex::Captures,
    capture_name: &str,
) -> Result<u32, String> {
    let string = regex_match.name(capture_name).unwrap().as_str();
    string
        .parse::<u32>()
        .map_err(|e| format!("Couldn't parse speed to u32: \"{}\"", string.to_owned()))
}
