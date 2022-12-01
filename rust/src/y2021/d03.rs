use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

const BITS: usize = 12;

pub fn p01(input: &String) -> String {
    let mut cnts: [u32; BITS] = [
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let mut threshold: u32 = 0;
    input.bytes().fold(0, |idx, c| {
        match c {
            0x30 => idx + 1,
            0x31 => {
                cnts[idx] += 1;
                idx + 1
            },
            _ => {
                threshold += 1;
                0
            }
        }
    });
    threshold = threshold >> 1;
    let gamma: u64 = cnts.iter().fold(0, |g, cnt| {
        if *cnt >= threshold {
            (g << 1) + 1
        } else {
            g << 1
        }
    });
    let epsilon = gamma ^ ((1 << BITS) - 1);
    return (gamma * epsilon).to_string();
}

pub fn p02(input: &String) -> String {
    let numbers: Vec<u32> = input.lines().map(|l| 
        u32::from_str_radix(l, 2).unwrap()
    ).collect();

    const STARTMASK: u32 = 1 << (BITS - 1);
    let o2gen = filter(&numbers, STARTMASK, true) as i64;
    let co2scr = filter(&numbers, STARTMASK, false) as i64;

    return (o2gen * co2scr).to_string();
}

type Split = (Vec<u32>, Vec<u32>);

fn filter(numbers: &Vec<u32>, mask: u32, most_frequent: bool) -> u32 {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let (zeros, ones): Split = numbers.iter().partition(|n| (*n & mask) == 0);
    let one_is_most_common = ones.len() >= zeros.len();
    if most_frequent == one_is_most_common {
        filter(&ones, mask >> 1, most_frequent)
    } else {
        filter(&zeros, mask >> 1, most_frequent)
    }
}