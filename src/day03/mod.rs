use itertools::Itertools;

pub fn part_a(input: &str) -> String {
    let compartments = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .collect::<Vec<_>>();

    let mut matches = Vec::new();
    for (a, b) in compartments {
        for char in a.chars() {
            if let Some(_) = b.find(char) {
                matches.push(char);
                break;
            }
        }
    }

    get_score(&matches).to_string()
}

pub fn part_b(input: &str) -> String {
    let groups = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|a| a.collect_tuple().unwrap())
        .collect::<Vec<(_, _, _)>>();

    let mut matches = Vec::new();
    for (a, b, c) in groups {
        for char in a.chars() {
            if let Some(_) = b.find(char) {
                if let Some(_) = c.find(char) {
                    matches.push(char);
                    break;
                }
            }
        }
    }

    get_score(&matches).to_string()
}

fn get_score(chars: &[char]) -> i32 {
    chars.iter().fold(0, |acc, char| {
        acc + match char {
            'A'..='Z' => *char as i32 - 38,
            'a'..='z' => *char as i32 - 96,
            _ => panic!("invalid char"),
        }
    })
}