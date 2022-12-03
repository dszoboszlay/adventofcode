use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

fn priority(c: u8) -> u8 {
    if c >= 'a' as u8 {
        c - 'a' as u8 + 1
    } else {
        c - 'A' as u8 + 27
    }
}

pub fn p01(input: &String) -> String {
    let res: u32 = input.lines().map(|l| {
        let l = l.as_bytes();
        let mut items: u64 = 0;
        let midpoint = l.len() / 2;
        let mut i = 0;
        while i < midpoint {
            items |= 1 << priority(l[i]);
            i += 1;
        }
        loop {
            let p = priority(l[i]);
            if (items & (1 << p)) != 0 {
                return p as u32;
            }
            i += 1;
        }
    }).sum();
    return res.to_string();
}

fn rucksack_to_set(l: &str) -> u64 {
    let mut items: u64 = 0;
    for c in l.as_bytes() {
        items |= 1 << priority(*c);
    }
    return items;
}

pub fn p02(input: &String) -> String {
    let mut iter = input.lines();
    let mut res: u32 = 0;
    loop {
        match iter.next() {
            None => { 
                return res.to_string();
            }
            Some(l) => {
                let badge = rucksack_to_set(l) &
                    rucksack_to_set(iter.next().unwrap()) &
                    rucksack_to_set(iter.next().unwrap());
                res += badge.trailing_zeros();
            }
        }
    }
}
