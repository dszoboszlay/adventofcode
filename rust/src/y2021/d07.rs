use crate::Day;
use std::cmp::min;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

pub fn p01(input: &String) -> String {
    const MAXPOS: usize = 2000;
    let mut submarines = [0; MAXPOS];
    let mut cnt = 0;

    input.lines().next().unwrap().split(',').for_each(|n| {
        submarines[n.parse::<usize>().unwrap()] += 1;
        cnt += 1;
    });

    let median = (cnt + 1) / 2;

    // Move all submarines below the median to the median pos
    let mut moving = 0;
    let mut fuel = 0;
    let mut idx = 0;
    while moving < median {
        fuel += moving;
        moving += submarines[idx];
        idx += 1;
    }

    // Now move all submarines above the median to the median
    moving = cnt - moving;
    while moving > 0 {
        fuel += moving;
        moving -= submarines[idx];
        idx += 1;
    }

    fuel.to_string()
}

pub fn p02(input: &String) -> String {
    const MAXPOS: usize = 2000;
    let mut submarines = [0; MAXPOS];
    let mut cost = [0; MAXPOS];
    let mut cnt = 0;

    input.lines().next().unwrap().split(',').for_each(|n| {
        submarines[n.parse::<usize>().unwrap()] += 1;
        cnt += 1;
    });

    let mut idx = 0;
    let mut moving = 0;
    let mut sum_cost = 0;
    let mut delta_cost = 0;
    while idx < MAXPOS {
        sum_cost += delta_cost;
        cost[idx] = sum_cost;
        moving += submarines[idx];
        delta_cost += moving;
        idx += 1;
    }

    moving = 0;
    sum_cost = 0;
    delta_cost = 0;
    let mut mincost = i64::MAX;
    while idx > 0 && sum_cost < mincost {
        idx -= 1;
        sum_cost += delta_cost;
        mincost = min(mincost, sum_cost + cost[idx]);
        moving += submarines[idx];
        delta_cost += moving;
    }

    mincost.to_string()
}
