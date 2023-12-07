#![feature(btree_cursors)]
#![feature(binary_heap_drain_sorted)]

use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub mod utils;
mod y2021;
mod y2022;
mod y2023;

pub type Solver = fn(&String) -> String;
pub type Day = (Option<Solver>, Option<Solver>);
pub type Year = HashMap<u16, Day>;
pub type Years = HashMap<u16, Year>;

fn main() {
    let years = HashMap::from([
        (2021, y2021::solvers()),
        (2022, y2022::solvers()),
        (2023, y2023::solvers()),
    ]);

    let measure_in_loop = env::var("NOLOOP").is_err();
    let mut args = env::args().skip(1);
    loop {
        let y = args.next();
        let d = args.next();
        let p = args.next();
        let f = args.next();

        match f {
            None => return,
            Some(f) => {
                let y: u16 = y.unwrap().parse().unwrap();
                let d: u16 = d.unwrap().parse().unwrap();
                let p: u16 = p.unwrap().parse().unwrap();
                let fp = Path::new(&f);
        
                let solver = years.get(&y).map(
                    |year| year.get(&d).map(
                        |(p1, p2)| match p {
                            1 => *p1,
                            _ => *p2
                        }
                    )
                ).flatten().flatten();
         
                match solver {
                    None => println!("N/A 0"),
                    Some(solver) => {
                        let input = fs::read_to_string(fp).unwrap();
                        let now = Instant::now();
                        let solution = solver(&input);
                        let mut t = now.elapsed().as_nanos();

                        if measure_in_loop {
                            // Aim for 5 ms runtime with looping
                            let loops = max(5_000_000 / max(10, t), 1);
                            let mut c = loops;
                            let now = Instant::now();
                            while c > 0 {
                                solver(&input);
                                c -= 1;
                            }
                            t = now.elapsed().as_nanos() / loops;
                        }

                        println!("{} {}", solution, t);
                    }
                }

            }
        }    
    }
}
