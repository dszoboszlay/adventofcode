use crate::Day;
use std::cmp::max;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let (last, best) = input.lines().fold((0, 0), |(elf, best), l| {
        if l.is_empty() {
            (0, max(elf, best))
        } else {
            (elf + l.parse::<i32>().unwrap(), best)
        }
    });
    return max(last, best).to_string();
}

type Best3 = (i32, i32, i32);

fn push(best: &mut Best3, next: i32) {
    if next <= best.2 {
        return;
    }

    if next > best.1 {
        best.2 = best.1;
        if next > best.0 {
            best.1 = best.0;
            best.0 = next;
        } else {
            best.1 = next;
        }
    } else {
        best.2 = next
    }
}

pub fn p02(input: &String) -> String {
    let mut best = (0, 0 ,0);
    let (last, best) = input.lines().fold((0, &mut best), |(elf, best), l| {
        if l.is_empty() {
            push(best, elf);
            (0, best)
        } else {
            (elf + l.parse::<i32>().unwrap(), best)
        }
    });
    push(best, last);
    return (best.0 + best.1 + best.2).to_string();
}