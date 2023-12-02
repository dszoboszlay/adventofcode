use std::cmp::max;
use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

#[derive(Debug)]
struct Reveal {
    r: u32,
    g: u32,
    b: u32,
}

type Game = Vec<Reveal>;
type Games = Vec<Game>;

fn parse(input: &String) -> Games {
    input.lines().map(|line| {
        let p = line.find(':').unwrap();
        line[(p + 1)..].split(';').map(|reveal| {
            let mut r = Reveal {r: 0, g: 0, b: 0};
            reveal.split(',').for_each(|s| {
                match s.chars().last().unwrap() {
                    'd' => r.r = s[1..(s.len() - 4)].parse().unwrap(),
                    'n' => r.g = s[1..(s.len() - 6)].parse().unwrap(),
                    'e' => r.b = s[1..(s.len() - 5)].parse().unwrap(),
                    _ => panic!("syntax error")
                }
            });
            r
        }).collect()
    }).collect()
}

pub fn p01(input: &String) -> String {
    let res: usize = parse(input).iter().enumerate().filter_map(|(i, game)| {
        if game.iter().all(|reveal| {
            reveal.r <= 12 && reveal.g <= 13 && reveal.b <= 14
        }) {
            Some(i + 1)
        } else {
            None
        }
    }).sum();

    return res.to_string()
}

pub fn p02(input: &String) -> String {
    let res: u32 = parse(input).iter().map(|game| {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        game.iter().for_each(|reveal| {
            r = max(r, reveal.r);
            g = max(g, reveal.g);
            b = max(b, reveal.b);
        });

        r * g * b
    }).sum();

    return res.to_string()
}
