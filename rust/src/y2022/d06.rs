use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let input = input.as_bytes();
    let mut i: usize = 3;

    loop {
        let a = input[i - 3];
        let b = input[i - 2];
        let c = input[i - 1];
        let d = input[i];

        if c == d {
            i += 3
        } else if b == c || b == d {
            i += 2
        } else if a == b || a == c || a == d {
            i += 1
        } else {
            return (i + 1).to_string();
        }
    }
}

pub fn p02(input: &String) -> String {
    let input = input.as_bytes();
    let mut i: usize = 0;

    'search: loop {
        let top = i + 14;
        let mut j = i + 12;
        while j >= i {
            if input[j + 1..top].iter().any(|x| *x == input[j]) {
                i = j + 1;
                continue 'search;
            }
            j -= 1;
        }

        return top.to_string();
    }
}
