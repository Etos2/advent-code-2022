mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use anyhow::{bail, ensure, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let args = std::env::args().skip(1);

    ensure!(
        args.len() % 2 == 0,
        "incorrect amount of arguments (expected {} found {})",
        args.len() + 1,
        args.len()
    );

    for (day, path) in args.tuples() {
        let data = std::fs::read_to_string(&path)?;
        let result = match day.to_ascii_lowercase().as_ref() {
            "day01a" => day01::part_a(&data),
            "day01b" => day01::part_b(&data),
            "day02a" => day02::part_a(&data),
            "day02b" => day02::part_b(&data),
            "day03a" => day03::part_a(&data),
            "day03b" => day03::part_b(&data),
            "day04a" => day04::part_a(&data),
            "day04b" => day04::part_b(&data),
            "day05a" => day05::part_a(&data),
            "day05b" => day05::part_b(&data),
            "day06a" => day06::part_a(&data),
            "day06b" => day06::part_b(&data),
            "day07a" => day07::part_a(&data),
            "day07b" => day07::part_b(&data),
            _ => bail!("challenge not completed or doesn't exist!"),
        };
        println!("{result}");
    }

    Ok(())
}
