use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let score: u64 = input.as_bytes().chunks(4).map(|l| {
        match (l[0], l[2]) {
            (65, 88) => 1+3,
            (65, 89) => 2+6,
            (65, 90) => 3+0,
            (66, 88) => 1+0,
            (66, 89) => 2+3,
            (66, 90) => 3+6,
            (67, 88) => 1+6,
            (67, 89) => 2+0,
            (67, 90) => 3+3,
            _ => 0
        }
    }).sum();
    return score.to_string();
}

pub fn p02(input: &String) -> String {
    let score: u64 = input.as_bytes().chunks(4).map(|l| {
        match (l[0], l[2]) {
            (65, 88) => 0+3,
            (65, 89) => 3+1,
            (65, 90) => 6+2,
            (66, 88) => 0+1,
            (66, 89) => 3+2,
            (66, 90) => 6+3,
            (67, 88) => 0+2,
            (67, 89) => 3+3,
            (67, 90) => 6+1,
            _ => 0
        }
    }).sum();
    return score.to_string();
}
