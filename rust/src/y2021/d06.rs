use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn solver(input: &String, days: u32) -> String {
    let mut fish: [i64; 9] = [0; 9];
    input.as_bytes().iter().step_by(2).for_each(|d| {
        fish[(*d - 0x30) as usize] += 1;
    });

    let mut days = days;
    while days != 0 {
        fish.rotate_left(1);
        fish[6] += fish[8];
        days -= 1;
    }
    
    fish.iter().sum::<i64>().to_string()
}

pub fn p01(input: &String) -> String {
    solver(input, 80)
}

pub fn p02(input: &String) -> String {
    solver(input, 256)
}
