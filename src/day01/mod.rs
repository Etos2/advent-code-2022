pub fn challenge_a(input: &str) {
    let max = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap();

    println!("{}", max);
}

pub fn challenge_b(input: &str) {
    let mut sums = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<_>>();

    sums.sort_by(|a, b| b.cmp(a));
    let max = sums.iter().take(3).sum::<i32>();

    println!("{}", max);
}
