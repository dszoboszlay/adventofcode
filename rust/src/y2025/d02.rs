use std::collections::BTreeSet;

use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
  let sum = input.split_terminator(&[',','\n'][..]).fold(0u64, |acc, range| {
    let mut acc = acc;
    let (l, h) = range.split_once('-').unwrap();
    let lo: u64 = l.parse().unwrap();
    let hi: u64 = h.parse().unwrap();

    let digits: u32 = (l.len() / 2) as u32;
    let mut half_limit = 10u64.pow(digits);
    let mut half_num: u64;

    if l.len() % 2 == 0 {
      half_num = l.split_at(digits as usize).0.parse().unwrap();
      if half_num * (half_limit + 1) < lo {
        half_num += 1
      }
    } else {
      half_num = half_limit;
      half_limit *= 10
    }

    loop {
      let n = half_num * (half_limit + 1);
      if n > hi {
        return acc
      }

      acc += n;
      half_num += 1;
      if half_num == half_limit {
        half_limit *= 10;
      }
    }
  });
  return sum.to_string()
}

pub fn p02(input: &String) -> String {
  let mut bounds: Vec<u64> = input.split_terminator(&[',','-','\n'][..]).map(|n|{
    return n.parse().unwrap();
  }).collect();
  bounds.sort();
  let hi = *bounds.last().unwrap();

  let mut invalid_ids = BTreeSet::new();
  let mut seq = 1u64;
  let mut mul = 10u64;
  loop {
    let mut n = seq * (mul + 1);
    if n > hi {
      return invalid_ids.iter().sum::<u64>().to_string()
    }

    while n <= hi {
      let idx = bounds.partition_point(|&x| x < n);
      if (idx % 2 == 1) || (idx < bounds.len() && bounds[idx] == n) {
        invalid_ids.insert(n);
      }

      n = n * mul + seq;
    }

    seq += 1;
    if seq == mul {
      mul *= 10;
    }
  }
}
