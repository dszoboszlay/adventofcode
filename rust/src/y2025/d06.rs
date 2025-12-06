use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
  let mut lines = input.lines().rev();
  let ops: Vec<u8> = lines.next().unwrap().split_whitespace().map(|w| w.bytes().next().unwrap()).collect();
  let mut res: Vec<u64> = ops.iter().map(|&op| { if op == '+' as u8 { 0u64} else { 1u64} }).collect();

  lines.for_each(|line| {
    line.split_whitespace().map(|n| n.parse::<u64>().unwrap()).enumerate().for_each(|(idx, n)| {
      if ops[idx] == '+' as u8 {
        res[idx] += n
      } else {
        res[idx] *= n
      }
    });
  });

  res.iter().sum::<u64>().to_string()
}

pub fn p02(input: &String) -> String {
  let cols = input.find('\n').unwrap();
  let line_width = cols + 1;
  let ops_offset = input.len() - line_width;
  let mut ops = input.as_bytes()[ops_offset..].iter().filter_map(|&c| {
    if c == '+' as u8 || c == '*' as u8 { Some(c) } else { None }
  });

  let numbers = input.split_at(ops_offset).0;
  let mut op = ops.next().unwrap();
  let mut acc = if op == '+' as u8 { 0u64 } else { 1u64 };
  let mut total = 0u64;
  for col in 0..cols {
    let n = numbers[col..].bytes().step_by(line_width).fold(0u64, |acc, c| {
      if c == ' ' as u8 {
        acc
      } else {
        acc * 10 + (c - '0' as u8) as u64
      }

    });

    if n == 0 {
      total += acc;
      op = ops.next().unwrap();
      acc = if op == '+' as u8 { 0u64 } else { 1u64 };
    } else if op == '+' as u8 {
      acc += n;
    } else {
      acc *= n;
    }
  }

  total += acc;
  total.to_string()
}
