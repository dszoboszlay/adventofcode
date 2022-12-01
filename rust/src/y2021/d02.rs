use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let (x, y) = input.lines().map(|l| match l.chars().next() {
        Some('d') => (0, l[5..].parse().unwrap()),           // down
        Some('u') => (0, -(l[3..].parse::<i64>().unwrap())), // up
        _         => (l[8..].parse().unwrap(), 0),           // forward
    }).reduce(|(x0, y0), (x1, y1)| {
        (x0 + x1, y0 + y1)
    }).unwrap();

    return (x * y).to_string();
}

pub fn p02(input: &String) -> String {
    let (x, y, _) = input.lines().map(|l| match l.chars().next() {
        Some('d') => (0, l[5..].parse().unwrap()),           // down
        Some('u') => (0, -(l[3..].parse::<i64>().unwrap())), // up
        _         => (l[8..].parse().unwrap(), 0),           // forward
    }).fold((0, 0, 0), |(x, y, aim), (dx, daim)| {
        (x + dx, y + dx * aim, aim + daim)
    });

    return (x * y).to_string();
}