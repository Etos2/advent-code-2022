mod day01;
mod day02;

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
            _ => bail!("challenge not complete yet!"),
        };
        println!("{result}");
    }

    Ok(())
}
