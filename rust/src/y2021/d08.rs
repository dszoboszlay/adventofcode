use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let mut cnt = 0;
    input.lines().for_each(|line| {
        let output = line.get(61..).unwrap();
        
        output
        .match_indices(' ')
        .map(|(idx, _)| idx)
        .chain([output.len()].into_iter())
        .fold(0, |last, curr| {
            match curr - last {
                2 => cnt += 1, // 1
                3 => cnt += 1, // 7
                4 => cnt += 1, // 4
                7 => cnt += 1, // 8
                _ => {},
            }
            curr + 1
        });
    });

    cnt.to_string()
}

pub fn p02(input: &String) -> String {
    #[derive(Debug, PartialEq)]
    enum State {
        Pattern,
        Separator,
        Output
    }
    let mut decode = [0 as u8; 128];
    let mut encode = [0 as u8; 10];
    let mut samples = [0 as u8; 7];
    let mut idx = 0 as usize;
    let mut output = 0 as i64;
    let mut sum = 0 as i64;
    
    input.bytes().fold(State::Pattern, |state, c| {
        match c {
            // |
            0x7c => {
                // Solve the remaining numbers

                // 9
                let pattern = encode[4] | encode[7];
                idx = samples.iter().position(|&sample| sample & pattern == pattern).unwrap();
                decode[samples[idx] as usize] = 9;
                encode[9] = samples[idx];
                samples[idx] = samples[5];

                // 0 and 3
                let pattern = encode[1];
                idx = samples.iter().position(|&sample| sample & pattern == pattern).unwrap();
                if samples[idx].count_ones() == 6 {
                    decode[samples[idx] as usize] = 0;
                    encode[0] = samples[idx];
                } else {
                    decode[samples[idx] as usize] = 3;
                    encode[3] = samples[idx];
                }
                samples[idx] = samples[4];
                idx = samples.iter().position(|&sample| sample & pattern == pattern).unwrap();
                if samples[idx].count_ones() == 6 {
                    decode[samples[idx] as usize] = 0;
                    encode[0] = samples[idx];
                } else {
                    decode[samples[idx] as usize] = 3;
                    encode[3] = samples[idx];
                }
                samples[idx] = samples[3];

                // 5
                let pattern = encode[8] ^ encode[9];
                idx = samples.iter().position(|&sample| sample & pattern == 0).unwrap();
                decode[samples[idx] as usize] = 5;
                encode[5] = samples[idx];
                samples[idx] = samples[2];

                // 2 and 6
                if samples[0].count_ones() == 5 {
                    decode[samples[0] as usize] = 2;
                    decode[samples[1] as usize] = 6;
                    encode[2] = samples[0];
                    encode[6] = samples[1];
                } else {
                    decode[samples[0] as usize] = 6;
                    decode[samples[1] as usize] = 2;
                    encode[6] = samples[0];
                    encode[2] = samples[1];
                }

                idx = 0;
                samples[0] = 0;
                State::Separator
            },
            // \s
            0x20 if state == State::Separator => State::Output,
            0x20 if state == State::Pattern => {
                // Look for numbers 1, 4, 7 & 8 which have unique bit counts
                match samples[idx].count_ones() {
                    2 => {
                        decode[samples[idx] as usize] = 1;
                        encode[1] = samples[idx];
                    },
                    3 => {
                        decode[samples[idx] as usize] = 7;
                        encode[7] = samples[idx];
                    },
                    4 => {
                        decode[samples[idx] as usize] = 4;
                        encode[4] = samples[idx];
                    },
                    7 => {
                        decode[samples[idx] as usize] = 8;
                        encode[8] = samples[idx];
                    },
                    _ => idx += 1
                };
                samples[idx] = 0;
                State::Pattern
            },
            0x20 if state == State::Output => {
                output = output * 10 + decode[samples[0] as usize] as i64;
                samples[0] = 0;
                State::Output
            },
            // \n
            0x0a => {
                // Add the last digit to the output and the output to the sum
                sum += output * 10 + decode[samples[0] as usize] as i64;

                // Reset
                output = 0;
                samples[0] = 0;
                State::Pattern
            },
            _ => {
                samples[idx] |= 1 << (c - 0x61);
                state
            }
        }
    });

    sum.to_string()
}
