use crate::Day;

const CHARS: usize = 'Z' as usize - 'A' as usize + 1;
const NODES: usize = CHARS * CHARS * CHARS;

fn parse_node(input: &str) -> usize {
    input.bytes().fold(0, |acc, c| acc * CHARS + (c - 'A' as u8) as usize)
}

struct DesertMap {
    instructions: Vec<usize>,
    starts: Vec<usize>,
    network: [[usize; 2]; NODES],
}

#[derive(Debug)]
struct Iter {
    targets: Vec<usize>,
    cycle_idx: usize,
    cycle: usize,
    idx: usize,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let val = self.targets[self.idx];
        self.idx += 1;
        if self.idx >= self.targets.len() {
            self.targets[self.cycle_idx..].iter_mut().for_each(|step| {
                *step += self.cycle;
            });
            self.idx = self.cycle_idx;
        }
        Option::Some(val)
    }
}

impl DesertMap {
    fn new(input: &String) -> DesertMap {
        let mut lines = input.lines();
        let instructions: Vec<usize> = lines.next().unwrap().chars().map(|c| {
            if c == 'L' {
                0
            } else {
                1
            }
        }).collect();
        lines.next();

        let mut starts = Vec::new();
        let mut network = [[0; 2]; NODES];
        for line in lines {
            let idx = parse_node(&line[0..3]);
            network[idx][0] = parse_node(&line[7..10]);
            network[idx][1] = parse_node(&line[12..15]);

            if line.chars().nth(2) == Some('A') {
                starts.push(idx);
            }
        }

        DesertMap {
            instructions: instructions,
            starts: starts,
            network: network,
        }
    }

    fn p01(&self) -> u64 {
        let mut step: u64 = 0;
        let mut instruction = self.instructions.iter().cycle();
        let mut pos: usize = 0;
        let target = self.network.len() - 1;

        while pos != target {
            pos = self.network[pos][*instruction.next().unwrap()];
            step += 1;
        }

        step
    }

    fn iter(&self, start: usize) -> Iter {
        let mut step: usize = 0;
        let mut instruction = self.instructions.iter().cycle();
        let mut pos = start;

        let mut targets: Vec<(usize, usize, usize)> = Vec::new();
        loop {
            pos = self.network[pos][*instruction.next().unwrap()];
            step += 1;

            if pos % CHARS == CHARS - 1 {
                let phase = step % self.instructions.len();
                if let Some((idx, &(_, s, _))) = targets.iter().enumerate().find(|(_, &(p, _, ph))| {
                    pos == p && phase == ph
                }) {
                    let cycle = step - s;

                    return Iter {
                        targets: targets.iter().map(|x| x.1).collect(),
                        cycle_idx: idx,
                        cycle: cycle,
                        idx: 0,
                    }
                } else {
                    targets.push((pos, step, phase));
                }
            }
        }
    }
}

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
    let map = DesertMap::new(input);

    return map.p01().to_string();
}

pub fn p02(input: &String) -> String {
    let map = DesertMap::new(input);
    let n = map.instructions.len();

    // This solution exploits some undocumented properties of the input I got
    // (but which I believe hold for all inputs).
    //
    // In the generic case if we start stepping from one of the starting positions,
    // there will be first some end positions that we encounter only once, but after
    // a while the same end positions will start to repeat. And by "end position" I
    // really mean a combination of a node that ends with Z and a position in the
    // instruction list.
    //
    // For example it may be that starting from 11A we can reach the following
    // end positions using a 100 element instruction list:
    // - 11Z after 42 steps
    // - 11Z after 53 steps
    // - 12Z after 83 steps
    // - 13Z after 159 steps
    // - 11Z after 253 steps
    //
    // After this point the end positions will start to repeat, because we already
    // reached 11Z 200 steps earlier. Note that the we don't start repeating end states
    // after step 53, even though we've already visited end node 11Z before, because at
    // that point we were at a different place in the instruction list.
    //
    // From these things it follows that once the end positions start to repeat, the cycle
    // length (in steps) has to be a multiple of the number of instructions.
    //
    // Now solving this problem in the general case would mean finding a step that appears
    // in the end point sequence of all the starting positions, which is hard.
    //
    // However, in the soecific input I have all starting positions can only reach a single
    // end position that will repeat forever. So the problem becomes much simpler: I only
    // have to find the least common multiple of the cycle lenghts. Which would in the
    // general case require the prime factorisation of the cycle lengths, but in my specific
    // input the cycle legngths are simply the number of instructions times a small,
    // unique prime, so calculating the least common multiple is trivial too.
    //
    // This solution exploits both of these particularities of my input without checking
    // that they hold.
    map.starts.iter().map(|&s| {
        map.iter(s).cycle / n
    }).fold(n, |acc, mul| {
        acc * mul
    }).to_string()
}
