use std::collections::BinaryHeap;
use crate::Day;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Hand {
    t: Type,
    cards: u32,
    bid: u64,
}

impl Hand {
    pub fn new1(input: &str) -> Hand {
        let mut cnt: [u32; 13] = [0; 13];
        let mut cards: u32 = 0;

        input[0..5].chars().for_each(|c| {
            let val = match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                _ => { (c as u32) - ('2' as u32) },
            };
            cards = (cards << 4) + val;
            cnt[val as usize] += 1;
        });

        cnt.sort_unstable_by(|a, b| b.cmp(a));
        let t = match cnt[0] {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => {
                if cnt[1] == 2 {
                    Type::FullHouse
                } else {
                    Type::ThreeOfAKind
                }
            },
            2 => {
                if cnt[1] == 2 {
                    Type::TwoPair
                } else {
                    Type::OnePair
                }
            },
            _ => Type::HighCard,
        };

        Hand {
            t: t,
            cards: cards,
            bid: input[6..].parse().unwrap()
        }
    }

    pub fn new2(input: &str) -> Hand {
        let mut cnt: [u32; 13] = [0; 13];
        let mut cards: u32 = 0;

        input[0..5].chars().for_each(|c| {
            let val = match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 0,
                'T' => 9,
                _ => { (c as u32) - ('1' as u32) },
            };
            cards = (cards << 4) + val;
            cnt[val as usize] += 1;
        });

        let j = cnt[0];
        cnt[0] = 0;
        cnt.sort_unstable_by(|a, b| b.cmp(a));
        let t = match cnt[0] + j {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => {
                if cnt[1] == 2 {
                    Type::FullHouse
                } else {
                    Type::ThreeOfAKind
                }
            },
            2 => {
                if cnt[1] == 2 {
                    Type::TwoPair
                } else {
                    Type::OnePair
                }
            },
            _ => Type::HighCard,
        };

        Hand {
            t: t,
            cards: cards,
            bid: input[6..].parse().unwrap()
        }
    }
}

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

pub fn p01(input: &String) -> String {
    let mut hands: BinaryHeap<Hand> = input.lines().map(|line| Hand::new1(line)).collect();
    let init = (hands.len() as u64, 0);
    let (_, res) = hands.drain_sorted().fold(init, |(rank, sum), hand| {
        (rank - 1, sum + rank * hand.bid)
    });
    res.to_string()
}

pub fn p02(input: &String) -> String {
    let mut hands: BinaryHeap<Hand> = input.lines().map(|line| Hand::new2(line)).collect();
    let init = (hands.len() as u64, 0);
    let (_, res) = hands.drain_sorted().fold(init, |(rank, sum), hand| {
        (rank - 1, sum + rank * hand.bid)
    });
    res.to_string()
}
