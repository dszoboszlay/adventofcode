use std::collections::HashMap;
use crate::Year;

mod d01;

pub fn solvers() -> Year {
    return HashMap::from([
        ( 1, d01::solvers()),
    ]);
}