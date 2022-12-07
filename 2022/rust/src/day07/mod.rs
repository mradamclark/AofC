pub mod input;
pub mod part_one;
pub mod part_two;

use std::collections::HashMap;
use nom::character::complete::u32;

use crate::{Part, Output};

pub type Input = HashMap<String, u32>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}

