use crate::Day;
use crate::utils::charmap::CharMap;

pub fn solvers() -> Day {
  (Some(p01),
   Some(p02),
  )
}

pub fn p01(input: &String) -> String {
  (CharMap::new(input, '.')).iter().filter(|field| {
    (*field) == '@' &&
    field.neighbours().iter().filter(|&&c| { c == '@' as u8}).count() < 4
  }).count().to_string()
}

pub fn p02(input: &String) -> String {
  let mut map = CharMap::new(input, '.');
  let mut cnt = 0;
  loop {
    let removed: Vec<(usize, usize)> = map.iter().filter(|field| {
      (*field) == '@' &&
      field.neighbours().iter().filter(|&&c| { c == '@' as u8}).count() < 4
    }).map(|field| field.coords()).collect();
    let n = removed.len();

    if n == 0 {
      break;
    }

    cnt += n;
    removed.iter().for_each(|&coords| {
      map[coords] = '.' as u8
    });
  }

  cnt.to_string()
}
