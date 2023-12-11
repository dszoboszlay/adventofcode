use std::collections::BTreeMap;
use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

type PosToCnt = BTreeMap<usize, usize>;

#[derive(Clone, Copy, Debug)]
struct Acc {
    sum_dist: usize,
    expansion: usize,
    last: usize,
    passed: usize,
    remaining: usize,
}

fn directional_distances_sum(map: &PosToCnt, cnt: usize, expansion_factor: usize) -> usize {
    let init = Acc {
        sum_dist: 0,
        expansion: 0,
        last: 0,
        passed: 0,
        remaining: cnt,
    };

    map.iter().fold(init, |mut acc, (&pos, &cnt)| {
        acc.expansion += (pos - acc.last - 1) *  expansion_factor;
        acc.last = pos;
        let pos = pos + acc.expansion;

        acc.remaining -= cnt;
        acc.sum_dist = acc.sum_dist.overflowing_add(acc.passed * cnt * pos).0.overflowing_sub(acc.remaining * cnt * pos).0;
        acc.passed += cnt;

        acc
    }).sum_dist
}

fn solve(input: &String, expansion_factor: usize) -> usize {
    let mut xs: PosToCnt = BTreeMap::new();
    let mut ys: PosToCnt = BTreeMap::new();
    let mut cnt: usize = 0;

    input.lines().enumerate().for_each(|(y, l)| {
        let cnt_in_l = l.bytes().enumerate().filter(|&(x, c)| {
            if c == '#' as u8 {
                xs.entry(x + 1).and_modify(|n| *n += 1).or_insert(1);
                true
            } else {
                false
            }
        }).count();

        if cnt_in_l > 0 {
            ys.insert(y + 1, cnt_in_l);
            cnt += cnt_in_l;
        }
    });

    directional_distances_sum(&xs, cnt, expansion_factor) + directional_distances_sum(&ys, cnt, expansion_factor)
}

pub fn p01(input: &String) -> String {
    solve(input, 1).to_string()
}

pub fn p02(input: &String) -> String {
    solve(input, 999_999).to_string()
}
