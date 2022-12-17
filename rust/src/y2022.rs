use std::collections::HashMap;
use crate::Year;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

pub fn solvers() -> Year {
    return HashMap::from([
        ( 1, d01::solvers()),
        ( 2, d02::solvers()),
        ( 3, d03::solvers()),
        ( 4, d04::solvers()),
        ( 5, d05::solvers()),
        ( 6, d06::solvers()),
        ( 7, d07::solvers()),
        ( 8, d08::solvers()),
        ( 9, d09::solvers()),
        (10, d10::solvers()),
    ]);
}