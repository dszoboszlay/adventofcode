use crate::Day;
use std::collections::HashSet;
use std::cmp::Ordering::*;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn follow(&mut self, head: &Pos) -> bool {
        if self.x == head.x + 2 {
            self.x -= 1;
            match head.y.cmp(&self.y) {
                Less => self.y -= 1,
                Equal => (),
                Greater => self.y += 1,
            }
            true
        } else if self.x == head.x - 2 {
            self.x += 1;
            match head.y.cmp(&self.y) {
                Less => self.y -= 1,
                Equal => (),
                Greater => self.y += 1,
            }
            true
        } else if self.y == head.y + 2 {
            self.y -= 1;
            match head.x.cmp(&self.x) {
                Less => self.x -= 1,
                Equal => (),
                Greater => self.x += 1,
            }
            true
        } else if self.y == head.y - 2 {
            self.y += 1;
            match head.x.cmp(&self.x) {
                Less => self.x -= 1,
                Equal => (),
                Greater => self.x += 1,
            }
            true
        } else {
            false
        }
    }

    fn step(&mut self, direction: char) {
        match direction {
            'U' => self.y -= 1,
            'D' => self.y += 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => panic!("invalid direction")
        }
    }
}

pub fn p01(input: &String) -> String {
    let mut pos_set: HashSet<Pos> = HashSet::new();
    let mut head = Pos {
        x: 0,
        y: 0,
    };
    let mut tail = head;
    pos_set.insert(tail);

    for l in input.lines() {
        let direction = l.chars().next().unwrap();
        let mut steps: u32 = l[2..].parse().unwrap();
        while steps > 0 {
            head.step(direction);
            if tail.follow(&head) {
                pos_set.insert(tail);
            }
            steps -= 1;
        }
    }

    return pos_set.len().to_string();
}

pub fn p02(input: &String) -> String {
    let mut pos_set: HashSet<Pos> = HashSet::new();
    let mut rope = [Pos{x: 0, y: 0}; 10];
    let tail: usize = 9;
    pos_set.insert(rope[tail]);

    for l in input.lines() {
        let direction = l.chars().next().unwrap();
        let mut steps: u32 = l[2..].parse().unwrap();
        while steps > 0 {
            rope[0].step(direction);
            for i in 1..tail {
                let prev = rope[i - 1];
                rope[i].follow(&prev);
            }
            let prev = rope[tail - 1];
            if rope[tail].follow(&prev) {
                pos_set.insert(rope[tail]);
            }

            steps -= 1;
        }
    }

    return pos_set.len().to_string();
}
