pub mod input;
pub mod part_one;
pub mod part_two;

use crate::Part;

pub type Input<'a> = Vec<&'a str>;

pub fn run(part: Part) -> u32 {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}
