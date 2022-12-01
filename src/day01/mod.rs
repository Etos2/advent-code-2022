use anyhow::{Context, Result};

pub fn challenge_a(input: &str) -> Result<()> {
    let sums = input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| {
                    l.parse::<i32>()
                        .with_context(|| format!("unable to parse \"{l}\""))
                })
                .sum::<Result<i32>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let max = sums.iter().max().with_context(|| "file is empty")?;

    println!("{}", max);
    Ok(())
}

pub fn challenge_b(input: &str) -> Result<()> {
    let mut sums = input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| {
                    l.parse::<i32>()
                        .with_context(|| format!("unable to parse \"{l}\""))
                })
                .sum::<Result<i32>>()
        })
        .collect::<Result<Vec<_>>>()?;

    sums.sort_by(|a, b| b.cmp(a));
    let max = sums.iter().take(3).sum::<i32>();

    println!("{}", max);
    Ok(())
}
