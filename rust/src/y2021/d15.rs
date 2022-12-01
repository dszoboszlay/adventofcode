use crate::Day;
use crate::utils::astar::*;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

struct Cavern<'a> {
    map: &'a [u8],
    x: usize,
    y: usize,
    dy: usize,
    tx: usize,
    ty: usize,
}

impl<'a> Cavern<'a> {
    fn risk_level(&self, x: usize, y: usize) -> usize {
        let tile = x / self.x + y / self.y;
        let offset = (y % self.y) * self.dy + (x % self.x);
        (((self.map[offset] - ('1' as u8)) as usize) + tile) % 9 + 1
    }

    fn new(s: &'a str, mul: usize) -> Cavern<'a> {
        let x = s.find('\n').unwrap();
        let dy = x + 1;
        let y = s.len() / dy;
        Cavern {
            map: s.as_bytes(),
            x: x,
            y: y,
            dy: dy,
            tx: x * mul - 1,
            ty: y * mul - 1,
        }
    }
}

struct CavernState<'a> {
    cavern: &'a Cavern<'a>,
    x: usize,
    y: usize,
    total_risk: usize,
}

impl<'a> CavernState<'a> {
    fn start(c: &'a Cavern<'a>) -> CavernState<'a> {
        CavernState {
            cavern: c,
            x: 0,
            y: 0,
            total_risk: 0,
        }
    }
}

impl<'a> SearchState for CavernState<'a> {
    type Key = (usize, usize);
    type Iter = CavernStateIterator<'a>;

    fn key(&self) -> Self::Key {
        (self.x, self.y)
    }

    fn is_goal(&self) -> bool {
        self.x == self.cavern.tx && self.y == self.cavern.ty
    }

    fn cost(&self) -> usize {
        self.total_risk
    }

    fn heuristic(&self) -> usize {
        self.cavern.tx + self.cavern.ty - self.x - self.y
    }

    fn next_states(self) -> Self::Iter {
        CavernStateIterator{
            state: self,
            dir: Direction::first()
        }

    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
    None,
}

impl Direction {
    fn first() -> Direction {
        Direction::Up
    }

    fn bump(&mut self)  {
        use Direction::*;
        *self = match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => None,
            None => None,
        };
    }
}

struct CavernStateIterator<'a> {
    state: CavernState<'a>,
    dir: Direction,
}

impl<'a> Iterator for CavernStateIterator<'a> {
    type Item = CavernState<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let xy = match self.dir {
            Direction::None => return None,
            Direction::Up if self.state.y > 0 => Some((self.state.x, self.state.y - 1)),
            Direction::Left if self.state.x > 0 => Some((self.state.x - 1, self.state.y)),
            Direction::Down if self.state.y < self.state.cavern.ty  => Some((self.state.x, self.state.y + 1)),
            Direction::Right if self.state.x < self.state.cavern.tx  => Some((self.state.x + 1, self.state.y)),
            _ => None
        };
        if let Some((x, y)) = xy {
            self.dir.bump();
            Some(CavernState{
                cavern: self.state.cavern,
                x: x,
                y: y,
                total_risk: self.state.total_risk + self.state.cavern.risk_level(x, y),
            })
        } else {
            self.dir.bump();
            self.next()
        }
    }
}


pub fn p01(input: &String) -> String {
    let cavern = Cavern::new(input, 1);
    solve(CavernState::start(&cavern)).unwrap().cost().to_string()
}

pub fn p02(input: &String) -> String {
    let cavern = Cavern::new(input, 5);
    solve(CavernState::start(&cavern)).unwrap().cost().to_string()
}
