use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

const TILE_BITS: usize = 3;
const TILE_SIZE: usize = 1 << TILE_BITS;
const MAP_SIZE: usize = 1000;
const MAP_TILE_SIZE: usize = (MAP_SIZE + TILE_SIZE - 1) / TILE_SIZE;
    
#[derive(Debug, PartialEq)]
struct BiBitMap {
    // Each coordinate is represented on 2 bits:
    // 00: empty
    // 01: exactly one line covering the coordinate
    // 10: two or more lines covering the coordinate
    // 11: invalid
    //
    // Layout of low bits on the TILE_SIZE*TILE_SIZE tile:
    //
    // 00 02 04 06 08 0A 0C 0E
    // 10 12 14 16 18 1A 1C 1E
    // 20 22 24 26 28 2A 2C 2E
    // 30 32 34 36 38 3A 3C 3E
    // 40 42 44 46 48 4A 4C 4E
    // 50 52 54 56 58 5A 5C 5E
    // 60 62 64 66 68 6A 6C 6E
    // 70 72 74 76 78 7A 7C 7E
    //
    // Moving a pattern one column right thus means shifting by 2 bits,
    // and moving one row down means shifting by 2*TILE_SIZE bits.
    data: [u128; MAP_TILE_SIZE * MAP_TILE_SIZE]
}

const X_SHIFT_BITS: usize = 2;
const Y_SHIFT_BITS: usize = 2 * TILE_SIZE;
const FULL_H_LINE: u128 = 0x_0000_0000_0000_0000_0000_0000_0000_5555;
const FULL_V_LINE: u128 = 0x_0001_0001_0001_0001_0001_0001_0001_0001;
const COLS_FROM_MASK: [u128; TILE_SIZE] = [
    0x_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
    0x_fffc_fffc_fffc_fffc_fffc_fffc_fffc_fffc,
    0x_fff0_fff0_fff0_fff0_fff0_fff0_fff0_fff0,
    0x_ffc0_ffc0_ffc0_ffc0_ffc0_ffc0_ffc0_ffc0,
    0x_ff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00,
    0x_fc00_fc00_fc00_fc00_fc00_fc00_fc00_fc00,
    0x_f000_f000_f000_f000_f000_f000_f000_f000,
    0x_c000_c000_c000_c000_c000_c000_c000_c000,
];
const COLS_TO_MASK: [u128; TILE_SIZE] = [
    0x_0003_0003_0003_0003_0003_0003_0003_0003,
    0x_000f_000f_000f_000f_000f_000f_000f_000f,
    0x_003f_003f_003f_003f_003f_003f_003f_003f,
    0x_00ff_00ff_00ff_00ff_00ff_00ff_00ff_00ff,
    0x_03ff_03ff_03ff_03ff_03ff_03ff_03ff_03ff,
    0x_0fff_0fff_0fff_0fff_0fff_0fff_0fff_0fff,
    0x_3fff_3fff_3fff_3fff_3fff_3fff_3fff_3fff,
    0x_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
];
const ROWS_FROM_MASK: [u128; TILE_SIZE] = [
    0x_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
    0x_ffff_ffff_ffff_ffff_ffff_ffff_ffff_0000,
    0x_ffff_ffff_ffff_ffff_ffff_ffff_0000_0000,
    0x_ffff_ffff_ffff_ffff_ffff_0000_0000_0000,
    0x_ffff_ffff_ffff_ffff_0000_0000_0000_0000,
    0x_ffff_ffff_ffff_0000_0000_0000_0000_0000,
    0x_ffff_ffff_0000_0000_0000_0000_0000_0000,
    0x_ffff_0000_0000_0000_0000_0000_0000_0000,
];
const ROWS_TO_MASK: [u128; TILE_SIZE] = [
    0x_0000_0000_0000_0000_0000_0000_0000_ffff,
    0x_0000_0000_0000_0000_0000_0000_ffff_ffff,
    0x_0000_0000_0000_0000_0000_ffff_ffff_ffff,
    0x_0000_0000_0000_0000_ffff_ffff_ffff_ffff,
    0x_0000_0000_0000_ffff_ffff_ffff_ffff_ffff,
    0x_0000_0000_ffff_ffff_ffff_ffff_ffff_ffff,
    0x_0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
    0x_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
];

impl BiBitMap {
    fn new() -> BiBitMap {
        BiBitMap {
            data: [0; MAP_TILE_SIZE * MAP_TILE_SIZE]
        }
    }

    /*
    fn print_tile_row(&self, tile: u128, y_offset: usize) {
        let mut tile: u128 = tile >> (Y_SHIFT_BITS * y_offset);
        let mut x_offset = 0;
        while x_offset < TILE_SIZE {
            let c = match tile & 0x3 {
                0b00 => '.',
                0b01 => '1',
                0b10 => '2',
                _    => '?',
            };
            print!("{}", c);

            tile >>= X_SHIFT_BITS;
            x_offset += 1;
        }
    }
    */

    fn add_tile(&mut self, idx: usize, pattern: u128) -> u32 {
        //let old = self.data[idx];

        // The number of new crosses is the number of 01  cells in the current
        // value of the tile in positions where the pattern also contains a bit.
        //
        // Which is the same as the number of common bits between the current
        // value of the tile and the pattern.
        let res = (self.data[idx] & pattern).count_ones();

        // Add the pattern to the tile and then turn all 0b11 fields to 0b10
        self.data[idx] += pattern;
        /*
        println!("  masked add: {:0128b}", old);
        println!("            + {:0128b}", pattern);
        println!("            = {:0128b}", self.data[idx]);
        println!("     10 mask: {:0128b}", pattern << 1);
        println!("            = {:0128b}", (self.data[idx] & (pattern << 1)));
        println!("   downshift: {:0128b}", (self.data[idx] & (pattern << 1)) >> 1);
        println!("        mask: {:0128b}", !((self.data[idx] & (pattern << 1)) >> 1));
        println!("            = {:0128b}", self.data[idx] & !((self.data[idx] & (pattern << 1)) >> 1));
        */
        self.data[idx] &= !((self.data[idx] & (pattern << 1)) >> 1);

        /*
        println!("@{} = {},{} add {}",
                 idx,
                 (idx % MAP_TILE_SIZE) * TILE_SIZE,
                 (idx / MAP_TILE_SIZE) * TILE_SIZE,
                 res);
        let mut y_offset = 0;
        while y_offset < TILE_SIZE {
            self.print_tile_row(old, y_offset);

            if y_offset == 3 {
                print!(" + ");
            } else {
                print!("   ");
            }

            self.print_tile_row(pattern, y_offset);

            if y_offset == 3 {
                print!(" = ");
            } else {
                print!("   ");
            }

            self.print_tile_row(self.data[idx], y_offset);
            println!("");

            y_offset += 1;
        }
        */
        res
    }

    fn add_h_line(&mut self, x: usize, y: usize, len: usize) -> u32 {
        //println!("horizontal {},{} -> {}", x, y, len);
        let x_tile = x >> TILE_BITS;
        let y_tile = y >> TILE_BITS;
        let x_offset = x & (TILE_SIZE - 1);
        let y_offset = y & (TILE_SIZE - 1);
        
        let full_line: u128 = FULL_H_LINE << (Y_SHIFT_BITS * y_offset);
        let mut idx = y_tile * MAP_TILE_SIZE + x_tile;

        if x_offset + len < TILE_SIZE {
            // Special case: the entire line segment falls within one tile,
            // and we have to mask columns from both sides of it
            let pattern = full_line
                & COLS_FROM_MASK[x_offset]
                & COLS_TO_MASK[x_offset + len - 1];
            return self.add_tile(idx, pattern);
        }

        // Add the first (partial) tile
        let mut cnt = self.add_tile(idx, full_line & COLS_FROM_MASK[x_offset]);
        let mut len = len - (TILE_SIZE - x_offset);

        // Add whole tiles
        while len >= TILE_SIZE {
            len -= TILE_SIZE;
            idx += 1;
            cnt += self.add_tile(idx, full_line);
        }

        // Add the last (partial) tile
        if len > 0 {
            idx += 1;
            cnt += self.add_tile(idx, full_line & COLS_TO_MASK[len - 1]);
        }
        
        cnt
    }

    fn add_v_line(&mut self, x: usize, y: usize, len: usize) -> u32 {
        //println!("vertical {},{} -> {}", x, y, len);
        let x_tile = x >> TILE_BITS;
        let y_tile = y >> TILE_BITS;
        let x_offset = x & (TILE_SIZE - 1);
        let y_offset = y & (TILE_SIZE - 1);
        
        let full_line: u128 = FULL_V_LINE << (X_SHIFT_BITS * x_offset);
        let mut idx = y_tile * MAP_TILE_SIZE + x_tile;

        if y_offset + len < TILE_SIZE {
            // Special case: the entire line segment falls within one tile,
            // and we have to mask columns from both sides of it
            let pattern = full_line
                & ROWS_FROM_MASK[y_offset]
                & ROWS_TO_MASK[y_offset + len - 1];
            return self.add_tile(idx, pattern);
        }

        // Add the first (partial) tile
        let mut cnt = self.add_tile(idx, full_line & ROWS_FROM_MASK[y_offset]);
        let mut len = len - (TILE_SIZE - y_offset);

        // Add whole tiles
        while len >= TILE_SIZE {
            len -= TILE_SIZE;
            idx += MAP_TILE_SIZE;
            cnt += self.add_tile(idx, full_line);
        }

        // Add the last (partial) tile
        if len > 0 {
            idx += MAP_TILE_SIZE;
            cnt += self.add_tile(idx, full_line & ROWS_TO_MASK[len - 1]);
        }
        
        cnt
    }
}

pub fn p01(input: &String) -> String {
    let mut map = BiBitMap::new();
    let mut cnt = 0;

    input.lines().for_each(|l| {
        let p1 = l.find(',').unwrap();
        let p2 = l.find(' ').unwrap();
        let p3 = p2 + 3; // the space after the ' -> ' arrow
        let p4 = l.rfind(',').unwrap();

        let x0: usize = l[..p1].parse().unwrap();
        let y0: usize = l[(p1 + 1)..p2].parse().unwrap();
        let x1: usize = l[(p3 + 1)..p4].parse().unwrap();
        let y1: usize = l[(p4 + 1)..].parse().unwrap();

        if x0 == x1 {
            // vertical line
            if y0 <= y1 {
                cnt += map.add_v_line(x0, y0, y1 - y0 + 1);
            } else {
                cnt += map.add_v_line(x0, y1, y0 - y1 + 1);
            }
        } else if y0 == y1 {
            // horizontal line
            if x0 <= x1  {
                cnt += map.add_h_line(x0, y0, x1 - x0 + 1);
            } else {
                cnt += map.add_h_line(x1, y0, x0 - x1 + 1);
            }
        }
    });
    
    cnt.to_string()
}

pub fn p02(_input: &String) -> String {
    String::new()
}
