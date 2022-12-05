use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

type Stacks = Vec<Vec<char>>;
type Moves = Vec<Move>;

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn parse(input: &String) -> (Stacks, Moves) {
    let mut stacks: Stacks = Vec::new();
    let mut moves: Moves = Vec::new();
    let mut ls = input.lines();

    // Parse stacks
    loop {
        let l = ls.next().unwrap();
        if (l.as_bytes())[1].is_ascii_digit() {
            // This is the separator line
            stacks.iter_mut().for_each(|stack| stack.reverse());
            ls.next();
            break;
        }

        l.chars().skip(1).step_by(4).enumerate().filter(|(_, c)| {
            *c != ' '
        }).for_each(|(i, c)| {
            if stacks.len() <= i {
                stacks.resize(i + 1, Vec::new());
            }
            stacks[i].push(c);
        })
    }

    // Parse moves
    for l in ls {
        let p0 = 5; // after "move "
        let p1 = l.find(" from").unwrap();
        let p2 = p1 + 6;
        let p3 = l.find(" to").unwrap();
        let p4 = p3 + 4;

        let n:    usize = l[p0..p1].parse().unwrap();
        let from: usize = l[p2..p3].parse().unwrap();
        let to:   usize = l[p4..  ].parse().unwrap();
        moves.push(Move { n: n, from: from - 1, to: to - 1 });
    }

    return (stacks, moves);
}

pub fn p01(input: &String) -> String {
    let (mut stacks, moves) = parse(input);

    for Move {n, from, to} in moves {
        for _ in 0..n {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    return stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
}

pub fn p02(input: &String) -> String {
    let (mut stacks, moves) = parse(input);

    for Move {n, from, to} in moves {
        let i = stacks[from].len() - n;
        let mut tail = stacks[from].split_off(i);
        stacks[to].append(&mut tail);
    }

    return stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
}
