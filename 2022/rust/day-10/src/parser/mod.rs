
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    *,
};
use Instruction::*;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    AddX(i32),
    NoOp,
}
impl Instruction {
    pub fn cycles(&self) -> u8 {
        match self {
            NoOp => 1,
            AddX(_) => 2,
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| NoOp),
            preceded(tag("addx "), complete::i32).map(|n| AddX(n)),
        )),
    )(input)?;

    Ok((input, instructions))
}
