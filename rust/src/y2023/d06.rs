use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_range(&self) -> (u64, u64) {
        // x * (time - x) >= distance
        // -x^2 + time*x - distance >= 0
        //
        // a = -1
        // b = time
        // c = -distance
        // (-b +- sqrt(b^2 - 4 ac)) / 2a
        // (b +- sqrt(b^2 + 4c)) / 2
        // (time +- sqrt(time*time - 4*distance)) / 2
        
        let d = ((self.time*self.time - 4*self.distance) as f64).sqrt();
        let t = self.time as f64;
        let x0 = ((t - d) / 2.0).ceil() as u64;
        let x1 = ((t + d) / 2.0).floor() as u64;

        (x0, x1)
    }
}

pub fn p01(input: &String) -> String {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times.split_whitespace().zip(distances.split_whitespace()).skip(1).map(|(time, distance)| {
        let race = Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap()
        };
        let (x0, x1) = race.winning_range();
        x1 - x0 + 1
    }).fold(1, |a, b| { a * b }).to_string()
}

fn parse_number_with_spaces(input: &str) -> u64 {
    input.bytes().filter(|&c| { c != ' ' as u8 }).fold(0, |acc, x| {
        acc * 10 + (x - '0' as u8) as u64
    })
}

pub fn p02(input: &String) -> String {
    let mut lines = input.lines();
    let time = parse_number_with_spaces(&(lines.next().unwrap())[10..]);
    let distance = parse_number_with_spaces(&(lines.next().unwrap())[10..]);

    let race = Race {
        time: time,
        distance: distance,
    };
    let (x0, x1) = race.winning_range();

    (x1 - x0 + 1).to_string()
}
