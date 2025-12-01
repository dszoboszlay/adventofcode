use std::collections::BTreeMap;
use std::ops::Bound;
use crate::Day;

type Range = (i64, i64);

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct RangeShift {
    top: i64,
    shift: i64,
}

#[derive(Debug)]
struct RangeDict(BTreeMap<i64, RangeShift>);

impl RangeDict {
    fn look_up(&self, i: i64) -> i64 {
        match self.0.upper_bound(Bound::Included(&i)).peek_prev() {
            Some((&key, &val)) if val.top >= i => i + val.shift,
            _ => i
        }
    }

    fn append_shifted_ranges(&self, range: Range, res: &mut Vec<Range>) {
        let mut min = range.0;
        let max = range.1;

        let mut cursor = self.0.upper_bound(Bound::Included(&min));
        while let Some((&bottom, &val)) = cursor.peek_prev() {
            let RangeShift {top, shift} = val;

            if bottom > max || min > top {
                res.push((min, max));
                return;
            } else if bottom > min {
                res.push((min, bottom - 1));
                min = bottom;
            }

            if top >= max {
                res.push((min + shift, max + shift));
                return;
            } else {
                res.push((min + shift, top + shift));
                min = top + 1;
                cursor.next();
            }
        }

        res.push((min, max));
    }

    fn add_range(&mut self, dest_start: i64, src_start: i64, length: i64) {
        self.0.insert(src_start, RangeShift {top: src_start + length - 1, shift: dest_start - src_start});
    }

    const fn new() -> RangeDict {
        RangeDict(BTreeMap::new())
    }
}

fn parse(input: &String) -> (Vec<i64>, Vec<RangeDict>) {
    let mut lines = input.lines();

    // First line: seeds
    let seeds: Vec<i64> = lines.next().unwrap()[7..].split_whitespace().map(|n| { n.parse().unwrap() }).collect();

    // Skip two lines (empty & dictionary name)
    lines.next();
    lines.next();

    let mut dicts: Vec<RangeDict> = Vec::new();
    let mut dict = RangeDict::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            dicts.push(dict);
            lines.next();
            dict = RangeDict::new();
        } else {
            let [dest_start, src_start, length] = line.split_whitespace().map(|n| {
                n.parse().unwrap()
            }).collect::<Vec<i64>>().try_into().unwrap();
            dict.add_range(dest_start, src_start, length);
        }
    };
    dicts.push(dict);

    (seeds, dicts)
}

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
    let (seeds, dicts) = parse(input);

    seeds.iter().map(|&seed| {
        dicts.iter().fold(seed, |i, dict| {
            dict.look_up(i)
        })
    }).min().unwrap().to_string()
}

pub fn p02(input: &String) -> String {
    let (seeds, dicts) = parse(input);

    seeds.chunks_exact(2).map(|seed_range| {
        let init_ranges = vec![(seed_range[0], seed_range[0] + seed_range[1] - 1)];
        let final_ranges = dicts.iter().fold(init_ranges, |in_ranges, dict| {
            let mut out_ranges: Vec<Range> = Vec::new();
            in_ranges.iter().for_each(|&in_range| { dict.append_shifted_ranges(in_range, &mut out_ranges)});
            out_ranges
        });

        final_ranges.iter().map(|&range| { range.0 }).min().unwrap()
    }).min().unwrap().to_string()
}
