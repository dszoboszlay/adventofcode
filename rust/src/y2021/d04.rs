use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

// Encode a position on a board as a single integer:
// - Bits 0-3: column
// - Bits 4-7: row
// - Bits 8..: board number
type BoardPos = usize;

// Store all the bingo board states in one large vector.
//
// Each board has 11 entries:
// - 0: holds the sum of all unmarked numbers
// - 1-5: the number of marked numbers in each column
// - 6-10: the number of marked numbers in each row

const DRAWS: usize = 100;
const BOARD_SIZE: usize = 5;
const INDEXES_PER_BOARD: usize = 1 + 2 * BOARD_SIZE;
const EMPTY_VEC: Vec<BoardPos> = Vec::new();

fn sum_index(pos: BoardPos) -> usize {
    (pos >> 8) * INDEXES_PER_BOARD
}

pub fn p01(input: &String) -> String {
    let mut index = [EMPTY_VEC; DRAWS];
    let mut state: Vec<u16> = vec![0; INDEXES_PER_BOARD];

    let mut lines = input.lines();
    let mut draws = lines.next().unwrap().split(',').map(|draw| {
        draw.parse::<u16>().unwrap()
    });
    lines.next(); // skip first empty line

    let mut pos: BoardPos = 0;
    lines.for_each(|line| {
        if line.len() == 0 {
            // Start a new board, advance pos to the beginning of the next table
            state.extend_from_slice(&[0; INDEXES_PER_BOARD]);
            pos = (pos & !0xff) + 0x100;
        } else {
            line.split_whitespace().for_each(|word| {
                let n: u16 = word.parse().unwrap();
                state[sum_index(pos)] += n;
                index[n as usize].push(pos);
                pos += 1;
            });
            pos = (pos & !0x0f) + 0x10;
        }
    });

    loop {
        let n = draws.next().unwrap();
        let res = index[n as usize].iter().find_map(|pos| {
            let sum_idx = sum_index(*pos);
            let col_idx = sum_idx + (*pos & 0x0f) + 1;
            let row_idx = sum_idx + ((*pos & 0xf0) >> 4) + 6;
            state[sum_idx] -= n;
            state[col_idx] += 1;
            state[row_idx] += 1;

            if state[col_idx] == BOARD_SIZE as u16 || state[row_idx] == BOARD_SIZE as u16 {
                Some(state[sum_idx])
            } else {
                None
            }
        });

        if res.is_some() {
            return ((res.unwrap() as u32) * (n as u32)).to_string();
        }
    }
}

pub fn p02(input: &String) -> String {
    let mut index = [EMPTY_VEC; DRAWS];
    let mut state: Vec<u16> = vec![0; INDEXES_PER_BOARD];

    let mut lines = input.lines();
    let draws = lines.next().unwrap().split(',').map(|draw| {
        draw.parse::<u16>().unwrap()
    });
    lines.next(); // skip first empty line

    let mut pos: BoardPos = 0;
    lines.for_each(|line| {
        if line.len() == 0 {
            // Start a new board, advance pos to the beginning of the next table
            state.extend_from_slice(&[0; INDEXES_PER_BOARD]);
            pos = (pos & !0xff) + 0x100;
        } else {
            line.split_whitespace().for_each(|word| {
                let n: u16 = word.parse().unwrap();
                state[sum_index(pos)] += n;
                index[n as usize].push(pos);
                pos += 1;
            });
            pos = (pos & !0x0f) + 0x10;
        }
    });

    const WON: u16 = 0;
    draws.fold(0, |last, n| {
        index[n as usize].iter().fold(last, |last, pos| {
            let sum_idx = sum_index(*pos);
            if state[sum_idx] == WON {
                return last;
            }

            let col_idx = sum_idx + (*pos & 0x0f) + 1;
            let row_idx = sum_idx + ((*pos & 0xf0) >> 4) + 6;
            state[sum_idx] -= n;
            state[col_idx] += 1;
            state[row_idx] += 1;

            if state[col_idx] == BOARD_SIZE as u16 || state[row_idx] == BOARD_SIZE as u16 {
                let score = (state[sum_idx] as i64) * (n as i64);
                state[sum_idx] = WON;
                score
            } else {
                last
            }
        })
    }).to_string()
}
