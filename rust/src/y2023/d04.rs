use std::collections::{BTreeSet, VecDeque};
use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
    let mut res: u32 = 0;
    input.lines().for_each(|l| {
        let mut score: u32 = 1;
        let winning: BTreeSet<u32> = l[10..40].split_whitespace().map(|num| { num.parse().unwrap() }).collect();

        l[42..].split_whitespace().for_each(|num| {
            let n: u32 = num.parse().unwrap();
            if winning.contains(&n) {
                score <<= 1;
            }
        });

        res += score >> 1;
    });

    res.to_string()
}

pub fn p02(input: &String) -> String {
    let mut res: u32 = 0;
    let mut copies: VecDeque<u32> = VecDeque::new();

    input.lines().for_each(|l| {
        let mul = if let Some(n) = copies.pop_front() {
            n
        } else {
            1
        };
        res += mul;

        let winning: BTreeSet<u32> = l[10..40].split_whitespace().map(|num| { num.parse().unwrap() }).collect();
        let matches = l[42..].split_whitespace().filter(|num| {
            let n: u32 = num.parse().unwrap();
            winning.contains(&n)
        }).count();

        if copies.len() < matches {
            copies.resize(matches, 1);
        }
        copies.iter_mut().take(matches).for_each(|n| { *n = *n + mul; });
    });

    res.to_string()
}
