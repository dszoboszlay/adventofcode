use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

fn extrapolate(mut seq: Vec<i64>, last: usize) -> i64 {
    let mut zeroes = true;
    let mut idx: usize = 0;
    while idx < last {
        let diff = seq[idx + 1] - seq[idx];
        zeroes = zeroes && diff == 0;
        seq[idx] = diff;
        idx += 1;
    }

    if zeroes {
        seq[last]
    } else {
        seq[last] + extrapolate(seq, last - 1)
    }
}

pub fn p01(input: &String) -> String {
    input.lines().map(|line| {
        let seq: Vec<i64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let last = seq.len() - 1;
        let res = extrapolate(seq, last);

        res
    }).sum::<i64>().to_string()
}

pub fn p02(input: &String) -> String {
    input.lines().map(|line| {
        let mut seq: Vec<i64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        seq.reverse();
        let last = seq.len() - 1;
        let res = extrapolate(seq, last);

        res
    }).sum::<i64>().to_string()
}
