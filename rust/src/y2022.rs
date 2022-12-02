use std::collections::HashMap;
use crate::Year;

mod d01;
mod d02;

pub fn solvers() -> Year {
    return HashMap::from([
        ( 1, d01::solvers()),
        ( 2, d02::solvers()),
    ]);
}