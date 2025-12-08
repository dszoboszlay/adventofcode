use crate::Day;
use crate::utils::charmap::CharMap;

pub fn solvers() -> Day {
  (Some(p01),
   Some(p02),
  )
}

pub fn p01(input: &String) -> String {
  let mut map = CharMap::new(input, '.');
  let mut pos = map.iter().find(|f| {
    *f == 'S'
  }).unwrap().down().coords();
  let mut cnt = 0u64;
  let mut backtrack = Vec::new();

  loop {
    match map[pos] as char {
      '^' => {
        backtrack.push((pos.0 + 1, pos.1));
        pos.0 -= 1;
        cnt += 1;
      }
      '|' => {
        match backtrack.pop() {
          None => return cnt.to_string(),
          Some(p) => pos = p,
        }
      }
      _other => {
        map[pos] = '|' as u8;
        pos.1 += 1;
        if pos.1 >= map.height() {
          match backtrack.pop() {
            None => return cnt.to_string(),
            Some(p) => pos = p,
          }
        }
      }
    }
  }
}

pub fn p02(input: &String) -> String {
  let line_step = input.find('\n').unwrap() + 1;

  let mut timelines = vec![1u64; input.len()];
  let input = input.as_bytes();
  let mut idx = input.len() - line_step;
  while input[idx] != 'S' as u8 {
    idx -= 1;
    if input[idx] == '^' as u8 {
      timelines[idx] = timelines[idx + line_step - 1] + timelines[idx + line_step + 1]
    } else {
      timelines[idx] = timelines[idx + line_step]
    }
  }

  timelines[idx].to_string()
}
