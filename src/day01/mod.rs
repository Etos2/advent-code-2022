pub fn part_a(input: &str) -> String {
    let sums = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<_>>();

    sums.iter().max().unwrap().to_string()
}

pub fn part_b(input: &str) -> String {
    let mut sums = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<_>>();

    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum::<i32>().to_string()
}
