pub mod input;
pub mod part_one;
pub mod part_two;

use std::str::FromStr;
use std::fmt;
use std::string::ToString;

use crate::Part;

#[derive(Clone, Copy, enum_utils::FromStr, strum_macros::Display)]
pub enum Direction {
    #[strum(ascii_case_insensitive)]
    Forward,
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Down
}

pub struct SubCommand {
    direction: Direction,
    amount: u32
}

impl fmt::Display for SubCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dir:{} Amt:{}", self.direction, self.amount)
    }
}

pub type Input = Vec<SubCommand>;

pub fn run(part: Part) -> u32 {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}
