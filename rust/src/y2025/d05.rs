use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
  let mut lines = input.lines();
  let mut ranges: Vec<(u64,u64)> = Vec::new();

  let mut line = lines.next().unwrap();
  while line.len() > 0 {
    let (lo, hi) = line.split_once('-').unwrap();
    ranges.push((lo.parse().unwrap(), hi.parse().unwrap()));
    line = lines.next().unwrap();
  }

  lines.map(|l| l.parse().unwrap()).filter(|n| {
    ranges.iter().any(|(lo, hi)| {
      lo <= n && n <= hi
    })
  }).count().to_string()
}

pub fn p02(input: &String) -> String {
  let mut ranges: Vec<(u64,u64)> = input.lines()
  .take_while(|l| l.len() > 0)
  .map(|l| {
    let (lo, hi) = l.split_once('-').unwrap();
    (lo.parse::<u64>().unwrap(), hi.parse::<u64>().unwrap())
  }).collect();

  ranges.sort_by(|a, b| {
    let o = a.0.cmp(&b.0);
    if o == core::cmp::Ordering::Equal {
      b.1.cmp(&a.1)
    } else {
      o
    }
  });

  ranges.iter().fold((0u64, 0u64), |(top, acc), &(lo, hi)| {
    if top >= hi {
      (top, acc)
    } else if top >= lo {
      (hi, acc + hi - top)
    } else {
      (hi, acc + hi - lo + 1)
    }
  }).1.to_string()
}
