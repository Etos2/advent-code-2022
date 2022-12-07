use itertools::Itertools;

pub fn part_a(input: &str) -> String {
    const MARKER_LEN: usize = 4;
    let mut acc = MARKER_LEN - 1;
    let chars = input.chars().collect::<Vec<_>>();
    let _marker = chars.windows(MARKER_LEN).find(|chars| {
        acc += 1;
        for c in chars.iter().combinations(2) {
            if c[0] == c[1] {
                return false;
            }
        }
        true
    }).unwrap();

    acc.to_string()
}

pub fn part_b(input: &str) -> String {
    const MARKER_LEN: usize = 14;
    let mut acc = MARKER_LEN - 1;
    let chars = input.chars().collect::<Vec<_>>();
    let _marker = chars.windows(MARKER_LEN).find(|chars| {
        acc += 1;
        for c in chars.iter().combinations(2) {
            if c[0] == c[1] {
                return false;
            }
        }
        true
    }).unwrap();

    acc.to_string()
}
