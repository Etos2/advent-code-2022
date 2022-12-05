use itertools::Itertools;

pub fn part_a(input: &str) -> String {
    let ranges = input
        .lines()
        .map(|l| {
            l.split(&[',', '-'])
                .tuples()
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(_, _)>>();

    let contained = ranges.iter().fold(0, |acc, &(a, b)| {
        if a.0 >= b.0 && a.1 <= b.1 {
            acc + 1
        } else if b.0 >= a.0 && b.1 <= a.1 {
            acc + 1
        } else {
            acc
        }
    });

    contained.to_string()
}

pub fn part_b(input: &str) -> String {
    let ranges = input
        .lines()
        .map(|l| {
            l.split(&[',', '-'])
                .tuples()
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(_, _)>>();

    let contained = ranges.iter().fold(0, |acc, &(a, b)| {
        if (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) {
            acc + 1
        } else if (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.0 && b.1 <= a.1) {
            acc + 1
        } else {
            acc
        }
    });

    contained.to_string()
}
