use crate::Day;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

type Dot = (u32, u32);

fn fold(dot: Dot, fold: &Dot) -> Dot {
    let (x, y) = dot;
    match *fold {
        (fx, 0) if fx > 0 && x > fx => (2 * fx - x, y),
        (0, fy) if fy > 0 && y > fy => (x, 2 * fy - y),
        _ => dot
    }
}

pub fn p01(input: &String) -> String {
    let mut dots: Vec<Dot> = Vec::new();
    let mut first_fold: Dot = (0, 0);

    enum Parser {
        Dots,
        FirstFold,
        Folds,
    }

    input.lines().fold(Parser::Dots, |state, line| {
        match state {
            Parser::Dots if line.is_empty() => Parser::FirstFold,
            Parser::Dots => {
                let (x, y) = line.split_once(',').unwrap();
                dots.push((x.parse().unwrap(), y.parse().unwrap()));
                state
            },
            Parser::FirstFold => {
                let n = line[13..].parse().unwrap();
                if line.chars().nth(11) == Some('x') {
                    first_fold = (n, 0);
                } else {
                    first_fold = (0, n);
                }
                Parser::Folds
            },
            _ => state,
        }
    });

    let dots: HashSet<_> = dots.iter().map(|dot| fold(*dot, &first_fold)).collect();
    dots.len().to_string()
}

pub fn p02(input: &String) -> String {
    let ocr: HashMap<usize, char> = HashMap::from([
        (0b_1110_1001_1110_1001_1001_1110, 'B'),
        (0b_0110_1001_1000_1000_1001_0110, 'C'),
        (0b_1111_1000_1110_1000_1000_1000, 'F'),
        (0b_0110_1001_1000_1011_1001_0111, 'G'),
        (0b_1001_1001_1111_1001_1001_1001, 'H'),
        (0b_0011_0001_0001_0001_1001_0110, 'J'),
        (0b_1110_1001_1001_1110_1000_1000, 'P'),
        (0b_1111_0001_0010_0100_1000_1111, 'Z'),
    ]);
    let mut dots: Vec<Dot> = Vec::new();
    let mut folds: Vec<Dot> = Vec::new();

    enum Parser {
        Dots,
        Folds,
    }

    input.lines().fold(Parser::Dots, |state, line| {
        match state {
            Parser::Dots if line.is_empty() => Parser::Folds,
            Parser::Dots => {
                let (x, y) = line.split_once(',').unwrap();
                dots.push((x.parse().unwrap(), y.parse().unwrap()));
                state
            },
            Parser::Folds => {
                let n = line[13..].parse().unwrap();
                if line.chars().nth(11) == Some('x') {
                    folds.push((n, 0));
                } else {
                    folds.push((0, n));
                }
                state
            },
        }
    });

    let dots: HashSet<_> = dots.iter().map(|dot| {
        folds.iter().fold(*dot, fold)
    }).collect();

    // OCR: decode 5*6 pixel chars to bitmaps, map bitmaps to chars
    let mut res = String::new();
    for c in 0..8 {
        let xo = c * 5;
        let mut bits: usize = 0;
        for y in 0..6 {
            for x in 0..4 {
                bits <<= 1;
                if dots.contains(&(xo + x, y)) {
                    bits += 1;
                }
            }
        }
        res.push(*ocr.get(&bits).unwrap());
    }

    res
}
