use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let mut depths = input.lines().map(|l| l.parse::<i64>().unwrap());
    let first = depths.next().unwrap();
    let (_, cnt) = depths.fold((first, 0), |(prev, cnt), curr| {
        if curr > prev {
            (curr, cnt + 1)
        } else {
            (curr, cnt)
        }
    });
    return cnt.to_string();
}

pub fn p02(input: &String) -> String {
    let mut depths = input.lines().map(|l| l.parse::<i64>().unwrap());
    let d0 = depths.next().unwrap();
    let d1 = depths.next().unwrap();
    let d2 = depths.next().unwrap();
    let (_, _, _, cnt) = depths.fold((d0, d1, d2, 0), |(d0, d1, d2, cnt), d3| {
        if d3 > d0 {
            (d1, d2, d3, cnt + 1)
        } else {
            (d1, d2, d3, cnt)
        }
    });
    return cnt.to_string();
}