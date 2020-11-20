use regex::Regex;
use std::cmp;
use std::{env, fs, process};

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32,
    cycle_dist: u32,
    cycle_time: u32,
}

struct Winner<'a> {
    name: &'a str,
    dist: u32,
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Failed to read input.txt.  Error:\n{}", err);
        process::exit(1);
    });

    let reindeers = parse_input(input_text).unwrap_or_else(|err| {
        eprintln!("Couldn't parse contents of input.txt.  Error:\n{}", err);
        process::exit(2);
    });

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No race time provided.");
        process::exit(3);
    }
    let race_time = match args[1].parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Couldn't parse rate time arg to u32: \"{}\"", args[1]);
            process::exit(4);
        }
    };

    match find_winner(&reindeers, race_time) {
        Some(w) => {
            println!("The winner is {} with distance {}!", w.name, w.dist);
        }
        None => {
            println!("There was no winner!");
        }
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
            cycle_dist: speed * flight_time,
            cycle_time: flight_time + rest_time,
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
        .map_err(|_| format!("Couldn't parse speed to u32: \"{}\"", string.to_owned()))
}

fn find_winner<'a>(reindeers: &'a Vec<Reindeer>, race_time: u32) -> Option<Winner<'a>> {
    let mut best: Option<Winner> = None;

    for reindeer in reindeers {
        let dist = reindeer_dist(reindeer, race_time);

        match &best {
            Some(w) => {
                if w.dist < dist {
                    best = new_winner(reindeer, dist);
                }
            }
            None => best = new_winner(reindeer, dist),
        }
    }

    best
}

fn new_winner<'a>(reindeer: &'a Reindeer, dist: u32) -> Option<Winner<'a>> {
    Some(Winner {
        name: &(reindeer.name),
        dist,
    })
}

fn reindeer_dist(reindeer: &Reindeer, time: u32) -> u32 {
    let full_cycles = time / reindeer.cycle_time;
    let full_dist = reindeer.cycle_dist * full_cycles;
    let remaining_time = time % reindeer.cycle_time;
    let remaining_used_time = cmp::min(remaining_time, reindeer.flight_time);
    let partial_dist = remaining_used_time * reindeer.speed;

    full_dist + partial_dist
}
