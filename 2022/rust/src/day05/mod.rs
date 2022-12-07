use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

pub mod input;
pub mod part_one;
pub mod part_two;

use crate::{Part, Output};
use input::{CargoStack, CargoStacks, Instruction};

pub type Input = (CargoStacks, Vec<Instruction>);

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}


