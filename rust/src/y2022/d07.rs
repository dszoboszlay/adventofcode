use crate::Day;
use std::collections::HashMap;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

#[derive(Debug)]
enum Node {
    File(usize),
    Dir(HashMap<String, Node>),
}

#[derive(PartialEq)]
enum ParseResult {
    ToParent,
    ToRoot,
    Eof,
}

type Lines<'a> = std::iter::Peekable<std::str::Lines<'a>>;

fn parse_cmd(pwd: &mut Node, lines: &mut Lines) -> ParseResult {
    while let Some(line) = lines.next() {
        if line == "$ ls" {
            parse_ls(pwd, lines)
        } else if line == "$ cd /" {
            return ParseResult::ToRoot
        } else if line == "$ cd .." {
            return ParseResult::ToParent
        } else {
            let dir_name = line[5..].to_string();
            if let Node::Dir(contents) = pwd {
                let result = match contents.get_mut(&dir_name) {
                    Some(dir) => parse_cmd(dir, lines),
                    None => {
                        let mut dir = Node::Dir(HashMap::new());
                        let res = parse_cmd(&mut dir, lines);
                        contents.insert(dir_name, dir);
                        res
                    },
                };
                if result == ParseResult::ToParent {
                    continue;
                }
                return result;
            } else {
                panic!("pwd is a file");
            }
        }
    }
    return ParseResult::Eof;
}

fn parse_ls(pwd: &mut Node, lines: &mut Lines) {
    if let Node::Dir(contents) = pwd {
        while let Some(line) = lines.next_if(|&l| l.as_bytes()[0] != '$' as u8) {
            let space = line.find(' ').unwrap();
            let node = if space == 3 && line[0..3] == *"dir" {
                Node::Dir(HashMap::new())
            } else {
                Node::File(line[..space].parse().unwrap())
            };
            contents.insert(line[space + 1..].to_string(), node);
        }
    } else {
        panic!("pwd is a file");
    }
}

fn p01_sum(node: &Node, sum: &mut usize) -> usize {
    match node {
        Node::File(size) => *size,
        Node::Dir(contents) => {
            let size = contents.values().fold(0, |s, n| s + p01_sum(n, sum));
            if size <= 100000 {
                *sum += size;
            }
            size
        }
    }
}

pub fn p01(input: &String) -> String {
    let mut root = Node::Dir(HashMap::new());
    let mut lines = input.lines().peekable();
    loop {
        match parse_cmd(&mut root, &mut lines) {
            ParseResult::ToParent => panic!("cd .. in /"),
            ParseResult::ToRoot => continue,
            ParseResult::Eof => {
                let mut sum: usize = 0;
                p01_sum(&root, &mut sum);
                return sum.to_string();
            },
        }
    }
}

fn p02_sizes(node: &Node, sizes: &mut Vec<usize>) -> usize {
    match node {
        Node::File(size) => *size,
        Node::Dir(contents) => {
            let size = contents.values().fold(0, |s, n| s + p02_sizes(n, sizes));
            sizes.push(size);
            size
        }
    }
}

pub fn p02(input: &String) -> String {
    let mut root = Node::Dir(HashMap::new());
    let mut lines = input.lines().peekable();
    loop {
        match parse_cmd(&mut root, &mut lines) {
            ParseResult::ToParent => panic!("cd .. in /"),
            ParseResult::ToRoot => continue,
            ParseResult::Eof => {
                let mut sizes: Vec<usize> = Vec::new();
                let total = p02_sizes(&root, &mut sizes);
                let free = 70_000_000 - total;
                let to_free = 30_000_000 - free;

                return sizes.iter().filter(|&&s| s >= to_free).min().unwrap().to_string();
            },
        }
    }
}
