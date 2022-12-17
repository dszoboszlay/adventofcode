use crate::Day;
use crate::utils::ocr::ocr_4x6;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let mut res = 0;
    let mut cycle = 1;
    let mut x = 1;
    for line in input.lines() {
        match line.chars().next().unwrap() {
            'n' => {
                if cycle % 40 == 20 {
                    res += cycle * x;
                }
                cycle += 1
            }
            'a' => {
                let op: i32 = line[5..].parse().unwrap();
                match cycle % 40 {
                    20 => res += cycle * x,
                    19 => res += (cycle + 1) * x,
                    _ => ()
                }
                cycle += 2;
                x += op;
            }
            _ => panic!("invalid opcode")
        }
    }

    return res.to_string();
}

fn paint(pixel: &mut char, col: i32, x: i32) {
    if (col - x).abs() <= 1 {
        *pixel = '#'
    }
}

fn step(row: &mut usize, col: &mut usize) {
    if *col < 39 {
        *col += 1;
    } else {
        *col = 0;
        *row += 1;
    }
}

pub fn p02(input: &String) -> String {
    let mut output = [[' '; 40]; 7];
    let mut x = 1;
    let mut row = 0;
    let mut col = 0;
    for line in input.lines() {
        paint(&mut output[row][col], col as i32, x);
        step(&mut row, &mut col);
        if line.chars().next().unwrap() == 'a' {
            paint(&mut output[row][col], col as i32, x);
            step(&mut row, &mut col);

            let op: i32 = line[5..].parse().unwrap();
            x += op;
        }
    }

    let mut res = String::new();
    for c in 0..8 {
        let col = c * 5;
        let mut sprite = String::new();
        for row in 0..6 {
            let r: String = (output[row][col..(col+4)]).iter().collect();
            sprite += &r;
        }
        res.push(ocr_4x6(sprite.as_str()).unwrap());
    }

    return res;
}
