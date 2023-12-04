use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

fn is_digit(c: u8) -> bool {
    c >= '0' as u8 && c <= '9' as u8
}

fn digit_val(c: u8) -> u32 {
    (c - '0' as u8) as u32
}

pub fn p01(input: &String) -> String {
    let mut res: u32 = 0;
    let line_step = input.find('\n').unwrap() + 1;
    let i = input.as_bytes();
    let l = i.len();

    let init: (u32, usize) = (0, 0);
    i.iter().enumerate().fold(init, |acc, (idx, &char)| {
        let (num, len) = acc;
        if is_digit(char) {
            (num * 10 + digit_val(char), len + 1)
        } else if num > 0 {
            let start_idx = idx - len;

            let prev_row =
                if start_idx > line_step {
                    i[(start_idx - line_step - 1)..(idx - line_step + 1)].iter()
                } else if start_idx == line_step {
                    i[0..(len + 1)].iter()
                } else {
                    i[0..0].iter()
                };
            let prev_col =
                if start_idx > 0 {
                    i[(start_idx - 1)..start_idx].iter()
                } else {
                    i[0..0].iter()
                };
            let next_col = i[idx..(idx + 1)].iter();
            let next_row = 
                if idx + line_step < l {
                    i[(start_idx + line_step - 1)..(idx + line_step + 1)].iter()
                } else if idx + line_step == l {
                    i[(start_idx + line_step - 1)..l].iter()
                } else {
                    i[0..0].iter()
                };

            if prev_row.chain(prev_col).chain(next_col).chain(next_row).any(|&c| {
                c != '\n' as u8 && c != '.' as u8 && !is_digit(c)
            }) {
                res += num
            }

            (0, 0)
        } else {
            (0, 0)
        }
    });

    return res.to_string()
}

fn part_num(i: &[u8], mut idx: usize) -> Option<u32> {
    if idx >= i.len() || !is_digit(i[idx]) {
        return None;
    }

    while idx > 0 && is_digit(i[idx - 1]) {
        idx -= 1
    }

    let mut n = digit_val(i[idx]);
    idx += 1;
    while idx < i.len() && is_digit(i[idx]) {
        n = n * 10 + digit_val(i[idx]);
        idx += 1
    }

    return Some(n)
}

pub fn p02(input: &String) -> String {
    let mut res: u32 = 0;
    let line_step = input.find('\n').unwrap() + 1;
    let i = input.as_bytes();

    i.iter().enumerate().filter(|(_, &c)| { c == '*' as u8 }).for_each(|(idx, _)| {
        let mut parts: u32 = 0;
        let mut gear_ratio: u32 = 1;

        if let Some(n) = part_num(i, idx.overflowing_sub(line_step).0) {
            parts += 1;
            gear_ratio *= n;
        } else {
            if let Some(n) = part_num(i, idx.overflowing_sub(line_step + 1).0) {
                parts += 1;
                gear_ratio *= n;
            }
            if let Some(n) = part_num(i, idx.overflowing_sub(line_step - 1).0) {
                parts += 1;
                gear_ratio *= n;
            }
        }
        if let Some(n) = part_num(i, idx.overflowing_sub(1).0) {
            parts += 1;
            gear_ratio *= n;
        }
        if let Some(n) = part_num(i, idx + 1) {
            parts += 1;
            gear_ratio *= n;
        }
        if let Some(n) = part_num(i, idx + line_step) {
            parts += 1;
            gear_ratio *= n;
        } else {
            if let Some(n) = part_num(i, idx + line_step - 1) {
                parts += 1;
                gear_ratio *= n;
            }
            if let Some(n) = part_num(i, idx + line_step + 1) {
                parts += 1;
                gear_ratio *= n;
            }
        }

        if parts == 2 {
            res += gear_ratio
        }
    });

    return res.to_string()
}
