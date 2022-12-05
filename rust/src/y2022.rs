use std::collections::HashMap;
use crate::Year;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

pub fn solvers() -> Year {
    return HashMap::from([
        ( 1, d01::solvers()),
        ( 2, d02::solvers()),
        ( 3, d03::solvers()),
        ( 4, d04::solvers()),
        ( 5, d05::solvers()),
    ]);
}