use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, not_line_ending, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use trees::{Node, Tree};

#[derive(Debug, PartialEq)]
enum StorageUnit<'a> {
    Root(),
    Directory(&'a str),
    File(&'a str, u32),
}

#[derive(Debug)]
enum Command<'a> {
    Change(&'a str),
    List(Vec<StorageUnit<'a>>),
}

pub fn part_a(input: &str) -> String {
    let (_, commands) = parse_all(input).unwrap();
    let filesystem = build_tree(commands);

    let mut sizes = Vec::new();
    let _ = get_size(filesystem.root(), &mut sizes);

    sizes.iter().filter(|&&num| num < 100000).sum::<u32>().to_string()
}

pub fn part_b(input: &str) -> String {
    let (_, commands) = parse_all(input).unwrap();
    let filesystem = build_tree(commands);

    let mut sizes = Vec::new();
    let _ = get_size(filesystem.root(), &mut sizes);
    let unused = 30000000 - (70000000 - sizes.iter().max().unwrap());

    sizes.sort();
    sizes.iter().filter(|&&num| num > unused).min().unwrap().to_string()
}

fn get_size(node: &Node<StorageUnit>, sizes: &mut Vec<u32>) -> u32 {
    match node.data() {
        StorageUnit::File(_, size) => *size,
        _ => {
        let size = node.iter().fold(0, |mut acc, n| {
            acc += get_size(n, sizes);
            acc
        });
        sizes.push(size);
        size
    },
    }
}

fn parse_storage(input: &str) -> IResult<&str, StorageUnit> {
    let (input, storage_str) = alt((tag("dir"), digit1))(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = not_line_ending(input)?;

    let storage = match storage_str {
        "dir" => StorageUnit::Directory(name),
        value => StorageUnit::File(name, value.parse::<u32>().unwrap()),
    };

    Ok((input, storage))
}

fn parse_list(input: &str) -> IResult<&str, Command> {
    let (input, _) = newline(input)?;
    let (input, storages) = separated_list1(newline, parse_storage)(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((input, Command::List(storages)))
}

fn parse_change(input: &str) -> IResult<&str, Command> {
    let (input, dir) = preceded(space1, alt((tag("/"), tag(".."), alpha1)))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, Command::Change(dir)))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command_str) = alt((tag("ls"), tag("cd")))(input)?;

    let (input, command) = match command_str {
        "ls" => parse_list(input)?,
        "cd" => parse_change(input)?,
        _ => panic!("Invalid command: {}", command_str),
    };

    Ok((input, command))
}

fn parse_all(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, _) = tag("$ ")(input)?;
    let (input, commands) = separated_list1(tag("$ "), parse_command)(input)?;

    Ok((input, commands))
}

fn build_tree<'a>(commands: Vec<Command<'a>>) -> Tree<StorageUnit<'a>> {
    let mut filesystem = vec![Tree::new(StorageUnit::Root())];

    for command in commands.into_iter().skip(1) {
        match command {
            Command::Change(name) => match name {
                ".." => {
                    let tree = filesystem.pop().unwrap();
                    filesystem.last_mut().unwrap().push_back(tree);
                }
                name => filesystem.push(Tree::new(StorageUnit::Directory(name))),
            },
            Command::List(storages) => {
                storages
                    .into_iter()
                    .filter(|s| match s {
                        StorageUnit::File(_, _) => true,
                        _ => false,
                    })
                    .for_each(|s| filesystem.last_mut().unwrap().push_back(Tree::new(s)));
            }
        }
    }

    while filesystem.len() > 1 {
        let tree = filesystem.pop().unwrap();
        filesystem.last_mut().unwrap().push_back(tree);
    }

    filesystem.into_iter().nth(0).unwrap()
}
