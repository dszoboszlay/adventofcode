use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

fn parse(input: &String) -> (i64, i64, i64, i64) {
    // target area: x=155..215, y=-132..-72\n
    //                0  1 2  3   4   5 6  7
    let i0 = input.find("x=").unwrap() + 2;
    let i1 = input.find("..").unwrap();
    let i2 = i1 + 2;
    let i3 = input.find(", y=").unwrap();
    let i4 = i3 + 4;
    let i5 = input.rfind("..").unwrap();
    let i6 = i5 + 2;
    let i7 = input.len() - 1;

    let x_min = input[i0..i1].parse().unwrap();
    let x_max = input[i2..i3].parse().unwrap();
    let y_min = input[i4..i5].parse().unwrap();
    let y_max = input[i6..i7].parse().unwrap();

    // The solvers are expecting the target area to be in the lower right quadrant
    assert_eq!(true, x_min > 0);
    assert_eq!(true, y_max < 0);
    
    (x_min, x_max, y_min, y_max)
}

struct YIter {
    target_min: i64,
    target_max: i64,
    steps: i64,
    steps_max: i64,
    min: i64,
    y: i64,
    yield_negative: bool,
}

impl YIter {
    fn new(y_min: i64, y_max: i64) -> YIter {
        YIter {
            target_min: -y_max,
            target_max: -y_min,
            steps: 1,
            steps_max: ((2 * (-y_min)) as f64).sqrt() as i64,
            min: -y_max,
            y: 1 - y_min,
            yield_negative: false,
        }
    }
}

impl Iterator for YIter {
    type Item = (i64, i64); // y velocity + number of steps

    fn next(&mut self) -> Option<Self::Item> {
        if self.yield_negative && self.y > 0 {
            // Yield a downwards shot
            self.yield_negative = false;
            Some((-self.y, self.steps))
        } else if self.y > self.min && self.y > 1 {
            // Yield an upwards shot
            self.y -= 1;
            self.yield_negative = true;
            Some((self.y - 1, 2 * self.y - 1 + self.steps))
        } else if self.y > self.min && self.y == 1 {
            // Yield a horizontal shot
            self.y = 0;
            Some((0, self.steps))
        } else if self.steps >= self.steps_max {
            None
        } else {
            // Increment number of steps
            self.steps += 1;
            let i = self.steps * (self.steps - 1);
            let j = (self.steps - 1) * (self.steps - 2);
            let k = 2 * self.steps;
            let l = k - 2;

            self.min = (2 * self.target_min - i + k - 1) / k;
            self.y = std::cmp::min((2 * self.target_max - i + k - 1) / k,
                                   (2 * self.target_min -j - 1) / l);

            if self.y >= self.min {
                self.yield_negative = true;
                Some((self.y - 1, 2 * self.y - 1 + self.steps))
            } else {
                self.next()
            }
        }
    }
}

pub fn p01(input: &String) -> String {
    let (_x_min, _x_max, y_min, y_max) = parse(input);
    let mut ys = YIter::new(y_min, y_max);
    let (y0, _steps) = ys.next().unwrap();
    let yt = (y0 + 1) * y0 / 2;

    yt.to_string()
}

pub fn p02(_input: &String) -> String {
    let ys = YIter::new(-10, -5);
    let mut test: Vec<_> = ys.collect();
    println!("{:?}", test);
    test.sort();
    println!("{:?}", test);
    // y + y+1 + ... + y+k = (2y + k) * (k + 1) / 2 = (k^2 + (2y + 1)k + 2y) / 2
    //
    // 
    // y + y+1 + y+2 = 3y + 3 >= y_min
    //                 3y + 3 <= y_max
    //                 2y + 1 < 
    String::new()
}
