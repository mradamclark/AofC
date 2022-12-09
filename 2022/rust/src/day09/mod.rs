pub mod input;
pub mod part_one;
pub mod part_two;

use crate::{Output, Part};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub type Input = Vec<(Direction, u32)>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}
