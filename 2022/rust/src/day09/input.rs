use itertools::Itertools;
use nom::character::complete::u8;

use crate::day09::{Direction, Input, Output};

const INPUT: &str = include_str!("../../../input/day09/input.txt");
pub fn read() -> Input {
    let (_, instructions) = instruction_parser::parse(INPUT).unwrap();
    instructions
}

mod instruction_parser {
    use crate::day09::Direction;
    use clap::builder::TypedValueParser;
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::{self, alpha1, digit1, newline},
        combinator::verify,
        multi::{many1, separated_list1},
        sequence::separated_pair,
        *,
    };

    fn direction(input: &str) -> IResult<&str, Direction> {
        let (input, dir) = alt((
            complete::char('U').map(|_| Direction::Up),
            complete::char('R').map(|_| Direction::Right),
            complete::char('D').map(|_| Direction::Down),
            complete::char('L').map(|_| Direction::Left),
        ))(input)?;

        Ok((input, dir))
    }

    fn movement(input: &str) -> IResult<&str, (Direction, u32)> {
        let (input, dir) = separated_pair(direction, tag(" "), complete::u32)(input)?;

        Ok((input, dir))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
        let (input, movements) = separated_list1(newline, movement)(input)?;
        Ok((input, movements))
    }
}
