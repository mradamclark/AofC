pub mod input;
pub mod part_one;
pub mod part_two;

use crate::Part;

pub type Input<'a> = Vec<(&'a str, &'a str)>;

#[derive(Debug, Clone, Copy)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
pub enum STATE {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

pub fn run(part: Part) -> u32 {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}
