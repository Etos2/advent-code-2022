use itertools::Itertools;

#[rustfmt::skip]
const SCORE: [i32; 9] = [
    3, 0, 6,
    6, 3, 0,
    0, 6, 3,
];

pub fn part_a(input: &str) -> String {
    let mut score = 0;
    let rounds = input
        .lines()
        .map(|l| {
            l.split_terminator(' ')
                .map(|c| match c {
                    "A" | "X" => 1, // Rock
                    "B" | "Y" => 2, // Paper
                    "C" | "Z" => 3, // Scissor
                    _ => panic!("invalid digit {c}"),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    for (opponent, player) in rounds {
        let index = (opponent - 1) + (player - 1) * 3;
        score += player + SCORE[index as usize];
    }

    score.to_string()
}

pub fn part_b(input: &str) -> String {
    let mut score = 0;
    let rounds = input
        .lines()
        .map(|l| {
            l.split_terminator(' ')
                .map(|c| match c {
                    "A" | "X" => 1, // Rock / Lose
                    "B" | "Y" => 2, // Paper / Draw
                    "C" | "Z" => 3, // Scissor / Win
                    _ => panic!("invalid digit {c}"),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    for (opponent, outcome) in rounds {
        let player = match outcome {
            // Lose
            1 => {
                if opponent - 1 != 0 {
                    opponent - 1
                } else {
                    3
                }
            },
            // Draw
            2 => opponent,
            // Win
            3 => {
                if opponent + 1 != 4 {
                    opponent + 1
                } else {
                    1
                }
            },
            _ => unreachable!()
        };
        
        let index = (opponent - 1) + (player - 1) * 3;
        score += player + SCORE[index as usize];
    }

    score.to_string()
}
