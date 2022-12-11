use crate::handheld::Instruction;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    *,
};

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Instruction::NoOp),
            preceded(tag("addx "), complete::i32).map(|n| Instruction::AddX(n)),
        )),
    )(input)?;

    Ok((input, instructions))
}
