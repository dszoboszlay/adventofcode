use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

const START: i32 = 50;
const N: i32 = 100;

pub fn p01(input: &String) -> String {
    let (_, cnt) = input.lines().fold((START, 0), |(pos, cnt), line| {
      let (dir, step) = line.split_at(1);
      let mut i: i32 = step.parse().unwrap();

      if dir == "L" {
        i = -i;
      }

      let p = (((pos + i) % N) + N) % N;
      let c = if p == 0 { cnt + 1 } else { cnt };
      return (p, c)
    });

    cnt.to_string()
}

pub fn p02(input: &String) -> String {
    let (_, cnt) = input.lines().fold((START, 0), |(pos, cnt), line| {
      let (dir, step) = line.split_at(1);
      let mut i: i32 = step.parse().unwrap();

      let mut p = pos;
      let mut c = cnt;
      if dir == "L" {
        if i < p {
          // We can't even turn until 0
          return (p - i, cnt)
        }

        // Turn until 0
        if p > 0 {
          i -= p;
          c += 1;
        }

        // Do whole turns
        c += i / N;
        i %= N;

        // Turn any remaining clicks
        if i > 0 {
          p = N - i;
        } else {
          p = 0
        }
      } else {
        if i < N - p {
          // We can't even turn until 0
          return (p + i, cnt)
        }

        // Turn until 0
        if p > 0 {
          i -= N - p;
          c += 1;
        }

        // Do whole turns
        c += i / N;
        i %= N;

        // Turn any remaining clicks
        p = i;
      }

      return (p, c)
    });

    cnt.to_string()
}
