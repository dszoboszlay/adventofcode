use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

const NS: u8 = '|' as u8;
const EW: u8 = '-' as u8;
const NE: u8 = 'L' as u8;
const NW: u8 = 'J' as u8;
const SW: u8 = '7' as u8;
const SE: u8 = 'F' as u8;
const ST: u8 = 'S' as u8;
const NL: u8 = '\n' as u8;
const FLAG: u8 = 0x80;

pub fn p01(input: &String) -> String {
    let row_step = input.lines().next().unwrap().len() + 1;
    let map = input.as_bytes();
    let start = map.iter().enumerate().find(|&(_, c)| *c == ST).unwrap().0;

    let mut steps: usize = 1;
    let mut pos = start + row_step;
    let mut coming_from = Direction::North;

    if start > row_step && match map[start - row_step] {
        NS => true,
        SW => true,
        SE => true,
        _  => false
    } {
        pos = start - row_step;
        coming_from = Direction::South;
    }
    if start > 0 && match map[start - 1] {
        EW => true,
        NE => true,
        SE => true,
        _  => false
    } {
        pos = start - 1;
        coming_from = Direction::East;
    }
    if start < map.len() - 1 && match map[start + 1] {
        EW => true,
        NW => true,
        SW => true,
        _  => false
    } {
        pos = start + 1;
        coming_from = Direction::West;
    }

    while pos != start {
        (pos, coming_from) = match (map[pos], coming_from) {
            (NS, Direction::South) => (pos - row_step, Direction::South),
            (NS, Direction::North) => (pos + row_step, Direction::North),
            (EW, Direction::East) => (pos - 1, Direction::East),
            (EW, Direction::West) => (pos + 1, Direction::West),
            (NE, Direction::North) => (pos + 1, Direction::West),
            (NE, Direction::East) => (pos - row_step, Direction::South),
            (NW, Direction::North) => (pos - 1, Direction::East),
            (NW, Direction::West) => (pos - row_step, Direction::South),
            (SW, Direction::South) => (pos - 1, Direction::East),
            (SW, Direction::West) => (pos + row_step, Direction::North),
            (SE, Direction::South) => (pos + 1, Direction::West),
            (SE, Direction::East) => (pos + row_step, Direction::North),
            _ => panic!("lost our way")
        };

        steps += 1;
    }

    return (steps / 2).to_string()
}

pub fn p02(input: &String) -> String {
    let row_step = input.lines().next().unwrap().len() + 1;
    let mut map: Vec<u8> = Vec::from(input.as_bytes());
    let start = map.iter().enumerate().find(|&(_, c)| *c == ST).unwrap().0;

    let mut pos = start + row_step;
    let mut coming_from = Direction::North;

    if start > row_step && match map[start - row_step] {
        NS => true,
        SW => true,
        SE => true,
        _  => false
    } {
        pos = start - row_step;
        coming_from = Direction::South;
    }
    if start > 0 && match map[start - 1] {
        EW => true,
        NE => true,
        SE => true,
        _  => false
    } {
        pos = start - 1;
        coming_from = Direction::East;
    }
    if start < map.len() - 1 && match map[start + 1] {
        EW => true,
        NW => true,
        SW => true,
        _  => false
    } {
        pos = start + 1;
        coming_from = Direction::West;
    }

    let start_direction = coming_from;
    while pos != start {
        let tile = map[pos];
        map[pos] += FLAG;
        (pos, coming_from) = match (tile, coming_from) {
            (NS, Direction::South) => (pos - row_step, Direction::South),
            (NS, Direction::North) => (pos + row_step, Direction::North),
            (EW, Direction::East) => (pos - 1, Direction::East),
            (EW, Direction::West) => (pos + 1, Direction::West),
            (NE, Direction::North) => (pos + 1, Direction::West),
            (NE, Direction::East) => (pos - row_step, Direction::South),
            (NW, Direction::North) => (pos - 1, Direction::East),
            (NW, Direction::West) => (pos - row_step, Direction::South),
            (SW, Direction::South) => (pos - 1, Direction::East),
            (SW, Direction::West) => (pos + row_step, Direction::North),
            (SE, Direction::South) => (pos + 1, Direction::West),
            (SE, Direction::East) => (pos + row_step, Direction::North),
            _ => panic!("lost our way")
        };
    }

    map[start] = FLAG + match (start_direction, coming_from) {
        (Direction::North, Direction::North) => NS,
        (Direction::North, Direction::East) => SE,
        (Direction::North, Direction::West) => SW,
        (Direction::East, Direction::North) => NW,
        (Direction::East, Direction::East) => EW,
        (Direction::East, Direction::South) => SW,
        (Direction::South, Direction::East) => NE,
        (Direction::South, Direction::South) => NS,
        (Direction::South, Direction::West) => NW,
        (Direction::West, Direction::North) => NE,
        (Direction::West, Direction::South) => SE,
        (Direction::West, Direction::West) => EW,
        _ => panic!("invalid start directions")
    };

    map.iter().fold((false, 0), |(inside, cnt), &tile| {
        // inside holds whether we're inside or outside the area enclosed by the loop
        //
        // while moving along a horizontal section of the loop, this variable holds
        // whether the North side of the edge is inside or not
        match tile {
            _ if tile > FLAG => (match tile - FLAG {
                NS => !inside,
                EW => inside,
                NE => !inside,
                NW => !inside,
                SW => inside,
                SE => inside,
                _ => panic!("invalid marker")
            }, cnt),
            NL => (false, cnt),
            _ if inside => (inside, cnt + 1),
            _ => (inside, cnt),
        }
    }).1.to_string()
}
