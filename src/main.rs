mod day01;

use anyhow::{bail, Result, ensure};
use itertools::Itertools;
use strum_macros::EnumString;

use std::str::FromStr;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum Challenge {
    Day01a,
    Day01b,
    Day02a,
    Day02b,
    Day03a,
    Day03b,
    Day04a,
    Day04b,
    Day05a,
    Day05b,
    Day06a,
    Day06b,
    Day07a,
    Day07b,
    Day08a,
    Day08b,
    Day09a,
    Day09b,
    Day10a,
    Day10b,
    Day11a,
    Day11b,
    Day12a,
    Day12b,
    Day13a,
    Day13b,
    Day14a,
    Day14b,
    Day15a,
    Day15b,
    Day16a,
    Day16b,
    Day17a,
    Day17b,
    Day18a,
    Day18b,
    Day19a,
    Day19b,
    Day20a,
    Day20b,
    Day21a,
    Day21b,
    Day22a,
    Day22b,
    Day23a,
    Day23b,
    Day24a,
    Day24b,
    Day25a,
    Day25b,
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1);

    ensure!(
        args.len() % 2 == 0,
        "incorrect amount of arguments (expected {} found {})",
        args.len() + 1,
        args.len()
    );

    for (day, path) in args.tuples() {
        if let Ok(c) = Challenge::from_str(&day) {
            let data = std::fs::read_to_string(&path)?;
            match c {
                Challenge::Day01a => day01::challenge_a(&data)?,
                Challenge::Day01b => day01::challenge_b(&data)?,
                _ => bail!("challenge not complete yet!"),
            }
        }
    }

    Ok(())
}
