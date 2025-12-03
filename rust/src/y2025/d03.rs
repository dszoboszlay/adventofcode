use crate::Day;
use std::cmp;
use std::mem;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
  return input.lines().fold(0u64, |acc, line|{
    let mut chars = line.as_bytes().iter().rev();
    let b0 = *chars.next().unwrap();
    let a0 = *chars.next().unwrap();
    let (a, b) = chars.fold((a0, b0), |(a, b), &x| {
      if x >= a {
        (x, cmp::max(a, b))
      } else {
        (a, b)
      }
    });

    return acc + ((a - '0' as u8) * 10 + (b - '0' as u8)) as u64
  }).to_string()
}

pub fn p02(input: &String) -> String {
  const N: usize = 12;
  return input.lines().fold(0u64, |acc, line|{
    let chars = line.as_bytes();
    let len = chars.len();
    let split = len - N;
    let mut buff: [u8; N] = [0; N];
    buff.clone_from_slice(&chars[split..len]);

    chars[0..split].iter().rev().for_each(|&x| {
      let mut x = x;
      let mut i = 0;
      while i < N && buff[i] <= x {
        mem::swap(&mut x, &mut buff[i]);
        i += 1;
      }
    });

    acc + buff.iter().fold(0, |acc, &c| {
      acc * 10 + (c - '0' as u8) as u64
    })
  }).to_string()
}
