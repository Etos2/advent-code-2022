mod day01;

use anyhow::Result;
use itertools::Itertools;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Challenge {
    Day01a,
    Day01b,
}

impl FromStr for Challenge {
    type Err = ();

    fn from_str(input: &str) -> Result<Challenge, Self::Err> {
        match input.to_lowercase().as_ref() {
            "day01a" => Ok(Challenge::Day01a),
            "day01b" => Ok(Challenge::Day01b),
            _ => Err(()),
        }
    }
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1);
    for (day, path) in args.tuples() {
        if let Ok(c) = Challenge::from_str(&day) {
            let data = std::fs::read_to_string(&path)?;
            match c {
                Challenge::Day01a => day01::challenge_a(&data),
                Challenge::Day01b => day01::challenge_b(&data),
            }
        }
    }
    Ok(())
}
