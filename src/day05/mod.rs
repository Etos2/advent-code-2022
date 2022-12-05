use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, digit1, multispace1, newline, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{IResult, InputIter};

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Crate(char);

pub fn part_a(input: &str) -> String {
    let (_, (mut crates, moves)) = parse_all(input).unwrap();

    for move_tm in moves {
        for _ in 0..move_tm.amount {
            let crate_tm = crates[move_tm.from].pop().unwrap();
            crates[move_tm.to].push(crate_tm);
        }
    }

    crates
        .iter()
        .filter_map(|vec| vec.last().map(|c| *c))
        .collect()
}

pub fn part_b(input: &str) -> String {
    let (_, (mut crates, moves)) = parse_all(input).unwrap();

    let mut temp = Vec::new();
    for move_tm in moves {
        for _ in 0..move_tm.amount {
            let crate_tm = crates[move_tm.from].pop().unwrap();
            temp.push(crate_tm);
        }

        for crate_tm in temp.iter().rev() {
            crates[move_tm.to].push(crate_tm);
        }

        temp.clear();
    }

    crates
        .iter()
        .filter_map(|vec| vec.last().map(|c| *c))
        .collect()
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            amount: amount as usize,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    ))
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, crate_tm) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    Ok((
        input,
        match crate_tm {
            "   " => None,
            value => Some(value),
        },
    ))
}

fn parse_all(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates) = separated_list1(newline, separated_list1(tag(" "), parse_crate))(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    let mut base = Vec::with_capacity(crates.len());
    for _ in 0..=crates.len() {
        base.push(Vec::new());
    }

    for vec in crates.iter().rev() {
        for (i, crate_tm) in vec.iter().enumerate() {
            if let Some(c) = crate_tm {
                base[i].push(*c);
            }
        }
    }

    Ok((input, (base, moves)))
}
