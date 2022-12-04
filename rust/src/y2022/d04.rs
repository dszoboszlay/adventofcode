use crate::Day;
use std::cmp::Ordering;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    return input.lines().filter(|l| {
        let p1 = l.find('-').unwrap();
        let p2 = l.find(',').unwrap();
        let p3 = l.rfind('-').unwrap();

        let l0: u32 = l[..p1].parse().unwrap();
        let h0: u32 = l[(p1 + 1)..p2].parse().unwrap();
        let l1: u32 = l[(p2 + 1)..p3].parse().unwrap();
        let h1: u32 = l[(p3 + 1)..].parse().unwrap();

        match l0.cmp(&l1) {
            Ordering::Less => h0 >= h1,
            Ordering::Equal => true,
            Ordering::Greater => h0 <= h1
        }
    }).count().to_string();
}

pub fn p02(input: &String) -> String {
    return input.lines().filter(|l| {
        let p1 = l.find('-').unwrap();
        let p2 = l.find(',').unwrap();
        let p3 = l.rfind('-').unwrap();

        let l0: u32 = l[..p1].parse().unwrap();
        let h0: u32 = l[(p1 + 1)..p2].parse().unwrap();
        let l1: u32 = l[(p2 + 1)..p3].parse().unwrap();
        let h1: u32 = l[(p3 + 1)..].parse().unwrap();

        match l0.cmp(&l1) {
            Ordering::Less => h0 >= l1,
            Ordering::Equal => true,
            Ordering::Greater => l0 <= h1
        }
    }).count().to_string();
}
