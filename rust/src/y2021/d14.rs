use crate::Day;
use std::cmp::max;
use std::cmp::min;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

const A: u8 = 'A' as u8;
const SIZE: usize = (('Z' as u8) - A + 1) as usize;

type Rules = [[usize; SIZE]; SIZE];
type Counts = [usize; SIZE];
type Sums = Vec<[[Counts; SIZE]; SIZE]>;

pub fn add_inserted(a: usize, b: usize, level: usize, rules: &Rules, sums: &mut Sums, counts: &mut Counts) {
    if sums[level][a][b][0] < usize::MAX {
        // Already calculated
        for i in 0..SIZE {
            counts[i] += sums[level][a][b][i];
        }
    } else {
        let c = rules[a][b];
        let next_level = level - 1;
        add_inserted(a, c, next_level, rules, sums, counts);
        counts[c] += 1;
        add_inserted(c, b, next_level, rules, sums, counts);

        for i in 0..SIZE {
            sums[level][a][b][i] = sums[next_level][a][c][i] + sums[next_level][c][b][i];
        }
        sums[level][a][b][c] += 1;
    }
}

pub fn solve(input: &String, level: usize) -> String {
    let mut rules: Rules = [[0; SIZE]; SIZE];
    let mut sums: Sums = vec![[[[usize::MAX; SIZE]; SIZE]; SIZE]; level];
    sums[0] = [[[0; SIZE]; SIZE]; SIZE];

    let mut lines = input.lines();
    let mut template = lines.next().unwrap().bytes().map(|c| (c - A) as usize);
    lines.next();
    lines.for_each(|line| {
        let mut bytes = line.bytes();
        let a = (bytes.next().unwrap() - A) as usize;
        let b = (bytes.next().unwrap() - A) as usize;
        let c = (bytes.last().unwrap() - A) as usize;

        rules[a][b] = c;
        sums[0][a][b][c] = 1;
    });

    let mut counts: Counts = [0; SIZE];
    let first = template.next().unwrap();
    counts[first] = 1;
    template.fold(first, |prev, next| {
        counts[next as usize] += 1;
        add_inserted(prev, next, level - 1, &rules, &mut sums, &mut counts);
        next
    });

    let (max_cnt, min_cnt) = counts.iter().fold((0, usize::MAX), |(max_acc, min_acc), &cnt| {
        if cnt > 0 {
            (max(max_acc, cnt), min(min_acc, cnt))
        } else {
            (max_acc, min_acc)
        }
    });
    (max_cnt - min_cnt).to_string()
}

pub fn p01(input: &String) -> String {
    solve(input, 10)
}

pub fn p02(input: &String) -> String {
    solve(input, 40)
}
