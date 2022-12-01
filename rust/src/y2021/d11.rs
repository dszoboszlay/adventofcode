use crate::Day;
use std::io::Write;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    const SIZE: usize = 10;
    const DY: usize = SIZE + 1;
    const I0: usize = DY + 1;

    // The actual borad is 10x10, but we use a 11x12+1 array,
    // so we can safely access all neighbours without doing bound checks:
    //
    // ...........
    //  1234567890.
    //  1234567890.
    //  ⋮
    //  1234567890.
    //  ...........
    //
    // Dots mark this safety space around the actual board.
    //
    // For each field's index 'i' all 8 neighbours of 'i' (from 'i - DY - 1' to
    // 'i + DY + 1') are within the array.
    const BUFF_LEN: usize = DY * (SIZE + 2) + 2;
    let mut board = [0 as u8; BUFF_LEN];

    // Copy the input into the board
    (&mut board[I0..]).write(input.as_bytes()).unwrap();

    // An octopus can gain up to 9 energy in a step (1 base increase +
    // 8 flashes from neighbouars). Value encoding:
    //
    // 0..9: regular energy level
    // 10..18: an octopus that will flash, but hasn't flashed yet in this step
    // 19..27: an octopus that has already flashed in this step
    const BASE: u8 = '0' as u8;
    const F_ADD: u8 = 9;
    const F_LO: u8 = '9' as u8 + 1;
    const F_HI: u8 = F_LO + F_ADD;

    let mut do_flash: bool;
    let mut idx: usize;
    let mut flashes: i64 = 0;
    for _step in 0..100 {
        // Bump all numbers
        do_flash = false;
        idx = I0;
        for _y in 0..SIZE {
            for _x in 0..SIZE {
                // First iteration of a step: the values are in the 1..9 range
                // for non-highlighted octopuses and F_HI.. for highlighted
                board[idx] = board[idx] + 1;
                match board[idx] {
                    F_LO => do_flash = true,
                    n if n >= F_HI => board[idx] = BASE + 1,
                    _ => (),
                }
                idx += 1;
            }
            idx += 1;
        }
        while do_flash {
            do_flash = false;

            idx = I0;
            for _y in 0..SIZE {
                for _x in 0..SIZE {
                    if board[idx] >= F_LO && board[idx] < F_HI {
                        board[idx - DY - 1] += 1;
                        board[idx - DY] += 1;
                        board[idx - DY + 1] += 1;
                        board[idx - 1] += 1;
                        board[idx] = F_HI;
                        board[idx + 1] += 1;
                        board[idx + DY - 1] += 1;
                        board[idx + DY] += 1;
                        board[idx + DY + 1] += 1;

                        flashes += 1;
                        do_flash = true;
                    }
                    idx += 1;
                }
                idx += 1;
            }
        }
    }
    flashes.to_string()
}

pub fn p02(input: &String) -> String {
    const SIZE: usize = 10;
    const DY: usize = SIZE + 1;
    const I0: usize = DY + 1;
    const BUFF_LEN: usize = DY * (SIZE + 2) + 2;
    let mut board = [0 as u8; BUFF_LEN];

    // Copy the input into the board
    (&mut board[I0..]).write(input.as_bytes()).unwrap();

    const BASE: u8 = '0' as u8;
    const F_ADD: u8 = 9;
    const F_LO: u8 = '9' as u8 + 1;
    const F_HI: u8 = F_LO + F_ADD;

    let mut all_flash: bool;
    let mut do_flash: bool;
    let mut idx: usize;
    let mut step = 0;
    loop {
        step += 1;
        // Bump all numbers
        do_flash = false;
        idx = I0;
        for _y in 0..SIZE {
            for _x in 0..SIZE {
                // First iteration of a step: the values are in the 1..9 range
                // for non-highlighted octopuses and F_HI.. for highlighted
                board[idx] = board[idx] + 1;
                match board[idx] {
                    F_LO => do_flash = true,
                    n if n >= F_HI => board[idx] = BASE + 1,
                    _ => (),
                }
                idx += 1;
            }
            idx += 1;
        }
        while do_flash {
            all_flash = true;
            do_flash = false;

            idx = I0;
            for _y in 0..SIZE {
                for _x in 0..SIZE {
                    if board[idx] >= F_LO && board[idx] < F_HI {
                        board[idx - DY - 1] += 1;
                        board[idx - DY] += 1;
                        board[idx - DY + 1] += 1;
                        board[idx - 1] += 1;
                        board[idx] = F_HI;
                        board[idx + 1] += 1;
                        board[idx + DY - 1] += 1;
                        board[idx + DY] += 1;
                        board[idx + DY + 1] += 1;

                        do_flash = true;
                    } else if board[idx] < F_LO {
                        all_flash = false;
                    }
                    idx += 1;
                }
                idx += 1;
            }

            if all_flash {
                return step.to_string();
            }
        }
    }
}
