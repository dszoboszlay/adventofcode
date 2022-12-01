use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    const C2R: u8 = '0' as u8 - 1;
    let board = input.as_bytes();
    let sx = board.iter().position(|c| *c == '\n' as u8).unwrap();
    let dx = sx + 1;
    let sy = board.len() / dx;
    let mut sum: i64 = 0;

    let mut idx: usize = 0;
    let mut y = 0;
    while y < sy {
        let mut x = 0;
        while x < sx {
            let c = board[idx];
            if (x == 0 || c < board[idx - 1]) &&
               (x == sx - 1 || c < board[idx + 1]) &&
               (y == 0 || c < board[idx - dx]) &&
               (y == sy - 1 || c < board[idx + dx]) {
                   sum += (c - C2R) as i64;
               }
            x += 1;
            idx += 1;
        }
        y += 1;
        idx += 1;
    }

    sum.to_string()
}

pub fn p02(input: &String) -> String {
    const C9: u8 = '9' as u8;
    const NL: u8 = '\n' as u8;
    let mut board: Vec<u8> = Vec::from(input.as_bytes());
    let dx = board.iter().position(|c| *c == NL).unwrap() + 1;
    let len = board.len();
    let mut top_basins = [0 as i64; 4];

    fn basin(b: &mut Vec<u8>, i: usize, len: usize, dx: usize) -> i64 {
        match b[i] {
            C9 => 0,
            NL => 0,
            _ => {
                b[i] = C9;

                let mut s = 1;
                if i > 0 {
                    s += basin(b, i - 1, len, dx);
                    if i > dx {
                        s += basin(b, i - dx, len, dx);
                    }
                }
                if i < len - 1 {
                    s += basin(b, i + 1, len, dx);
                    if i < len - 1 - dx {
                        s += basin(b, i + dx, len, dx);
                    }
                }

                s
            }
        }
    }

    let mut idx = 0;
    while idx < len {
        match board[idx] {
            C9 => (),
            NL => (),
            _ => {
                let n = basin(&mut board, idx, len, dx);
                let mut i = 3;
                while i > 0 && n > top_basins[i - 1] {
                    top_basins[i] = top_basins[i - 1];
                    i -= 1;
                }
                top_basins[i] = n;
            }
        }
        idx += 1;
    }
    
    (top_basins[0] * top_basins[1] * top_basins[2]).to_string()
}
