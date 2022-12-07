use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};
use std::borrow::BorrowMut;

use crate::day05::Input;

const INPUT: &str = include_str!("../../../input/day05/input.txt");

pub fn read() -> Input {

    // split the input data into two chunks.
    // 1. set cargo
    // 2. set of movement instructions.
    let (mut cargo_map, instructions) = INPUT.split_once("\n\n").unwrap();

    // parsing the map into a cargo stacks.
    let cargo_map = parse_cargo_map::parse(cargo_map).expect("failed to read cargo map");

    // parse the instructions.
    let cargo_movements: Vec<Instruction> = instructions
        .lines()
        .flat_map(parse_cargo_instr::parse)
        .collect();

    // Return the pair of parsed input sections
    (cargo_map, cargo_movements)
}

mod parse_cargo_map {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::satisfy,
        combinator::{map, value},
        multi::separated_list1,
        sequence::delimited,
        IResult,
    };
    

    fn cargo_item(s: &str) -> IResult<&str, char> {
        let mut label = satisfy(|c| c.is_ascii_uppercase());
        delimited(tag("["), label, tag("]"))(s)
    }

    fn cargo_row(s: &str) -> IResult<&str, Vec<Option<char>>> {
        let mut maybe_cargo = map(cargo_item, Some);
        let mut empty_space = value(None, tag("   "));
        let mut cargo_or_empty = alt((maybe_cargo, empty_space));
        separated_list1(tag(" "), cargo_or_empty)(s)
    }

    fn cargo_rows(s: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
        separated_list1(tag("\n"), cargo_row)(s)
    }

    pub fn parse(s: &str) -> Result<CargoStacks> {
        // turn data into objects to use.
        let (_, rows) = cargo_rows(s).map_err(|_| anyhow!("oops, cargo incorrect"))?;
        let mut stacks = CargoStacks::default();

        for row in rows.iter().rev() {
            for (idx, maybe_cargo) in row.iter().enumerate() {
                if let Some(label) = maybe_cargo {
                    stacks[idx + 1].push(*label);
                }
            }
        }

        Ok(stacks)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CargoStack {
    pub cargo: [char; 64],
    pub height: usize
}
impl Default for CargoStack {
    fn default() -> Self {
        Self {
            cargo: ['.'; 64],
            height: 0,
        }
    }
}
impl CargoStack {
    pub fn push(&mut self, ch: char) {
        self.cargo[self.height] = ch;
        self.height += 1;
    }

    pub fn pop(&mut self) -> char {
        if self.height == 0 {
            return '.'
        }

        self.height -= 1;
        self.cargo[self.height]
    }
}
impl Display for CargoStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..self.height {
            write!(f, "[{}] ", self.cargo[i])?
        }
        Ok(())
    }
}

// there are only 9 stacks in the input.
#[derive(Debug, Default, Clone)]
pub struct CargoStacks(pub [CargoStack; 9]);

impl Index<usize> for CargoStacks {
    type Output = CargoStack;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}
impl IndexMut<usize> for CargoStacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}
impl Display for CargoStacks{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.iter().fold(Ok(()), |result, cargostack| {
            result.and_then(|_| writeln!(f, "{}", cargostack))
        })
    }
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub amount: u32,
    pub from: u8,
    pub to: u8,
}
impl From<(u32, u8, u8)> for Instruction {
    fn from(value: (u32, u8, u8)) -> Self {
        Self {
            amount: value.0,
            from: value.1,
            to: value.2
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "move {} from {} to {}", self.amount, self.from, self.to)
    }
}

mod parse_cargo_instr {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::take_while,
        character::complete::{u8,u32},
        combinator::into,
        sequence::{preceded, tuple},
        IResult,
    };

    fn dest_stack(l: &str) -> IResult<&str, u8>{
        preceded(take_while(|c: char| c.is_alphabetic() || c.is_whitespace()), u8)(l)
    }

    fn from_stack(l: &str) -> IResult<&str, u8>{
        preceded(take_while(|c: char| c.is_alphabetic() || c.is_whitespace()), u8)(l)
    }

    fn cargo_to_move(l: &str) -> IResult<&str, u32>{
        preceded(take_while(|c: char| c.is_alphabetic() || c.is_whitespace()), u32)(l)
    }

    fn instruction(l: &str) -> IResult<&str, Instruction>{
        into(tuple((cargo_to_move, from_stack, dest_stack)))(l)
           
    }

    pub fn parse(l: &str) -> Result<Instruction>  {
        let (_, result) = instruction(l).map_err(|_| anyhow!("hmm bad instructions"))?;
        Ok(result)
    }
}
