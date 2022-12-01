use crate::Day;
use std::collections::HashMap;


pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    let mut graph = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        let v = graph.entry(a).or_insert(Vec::new());
        v.push(b);
        let v = graph.entry(b).or_insert(Vec::new());
        v.push(a);
    });

    let mut cnt = 0;
    let mut search_state = Vec::new();
    search_state.push(("", graph.remove("start").unwrap(), 0 as usize));
    while !search_state.is_empty() {
        let last = search_state.len() - 1;
        let (_, bs, idx) = &mut search_state[last];
        if *idx >= bs.len() {
            let (a, bs, _) = search_state.pop().unwrap();
            if !a.is_empty() {
                graph.insert(a, bs);
            }
        } else {
            let b = bs[*idx];
            *idx += 1;

            if b == "end" {
                cnt += 1;
            } else if b.chars().next().unwrap().is_ascii_uppercase() {
                search_state.push(("", graph.get(b).unwrap().clone(), 0));
            } else if let Some(cs) = graph.remove(b) {
                search_state.push((b, cs, 0));
            }
        }
    }

    cnt.to_string()
}

pub fn p02(input: &String) -> String {
    let mut graph = HashMap::new();
    let mut visits: HashMap<_, _>;
    enum Visit {
        Unlimited,
        End,
        None,
        Once,
        Twice
    }

    fn initial_visit(cave: &str) -> Visit {
        if cave.chars().next().unwrap().is_ascii_uppercase() {
            Visit::Unlimited
        } else if cave == "start" {
            Visit::Twice // start cannot be revisited
        } else if cave == "end" {
            Visit::End
        } else {
            Visit::None
        }
    }

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        let v = graph.entry(a).or_insert(Vec::new());
        v.push(b);
        let v = graph.entry(b).or_insert(Vec::new());
        v.push(a);
    });
    visits = graph.iter().map(|(cave,_)| (*cave, initial_visit(cave))).collect();

    let mut cnt = 0;
    let mut search_state: Vec<(&str, &Vec<&str>, usize)> = Vec::new();
    let mut revisited = false;

    search_state.push(("start", graph.get("start").unwrap(), 0));
    while !search_state.is_empty() {
        let last = search_state.len() - 1;
        let (a, bs, idx) = &mut search_state[last];
        if *idx >= bs.len() {
            let visit = visits.get_mut(a).unwrap();
            match visit {
                Visit::Twice => { revisited = false; *visit = Visit::Once },
                Visit::Once => *visit = Visit::None,
                _ => ()
            };
            search_state.pop().unwrap();
        } else {
            let b = bs[*idx];
            *idx += 1;

            let visit = visits.get_mut(b).unwrap();
            match visit {
                Visit::Unlimited => search_state.push((b, graph.get(b).unwrap(), 0)),
                Visit::End => cnt += 1,
                Visit::None => {
                    *visit = Visit::Once; 
                    search_state.push((b, graph.get(b).unwrap(), 0)) 
                },
                Visit::Once if !revisited => {
                    revisited = true;
                    *visit = Visit::Twice; 
                    search_state.push((b, graph.get(b).unwrap(), 0)) 
                },
                _ => (),
            }
        }
    }

    cnt.to_string()
}
