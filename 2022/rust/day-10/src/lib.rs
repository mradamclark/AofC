use std::{collections::HashMap, fmt::Display, ops::Range};

use itertools::Itertools;

use crate::input_parser::Instruction;

mod input_parser {
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
}

pub fn process_part1(input: &str) -> String {
    let mut scores: HashMap<u32, i32> = HashMap::new();
    let watched_cycles = [20, 60, 100, 140, 180, 220];
    let (_, instructions) = input_parser::parse(input).unwrap();

    let mut cycle = 0;
    let mut x: i32 = 1;

    for instr in instructions.iter() {
        // println!("{:?} - c:{:?} x:{:?}", instr, cycle, x);
        if watched_cycles.contains(&(cycle + 1)) {
            scores.insert(cycle + 1, x * (cycle as i32 + 1));
        }

        if watched_cycles.contains(&(cycle + 2)) {
            scores.insert(cycle + 2, x * (cycle as i32 + 2));
        }

        // println!("{:?}", scores);

        cycle += instr.cycles() as u32;
        match instr {
            Instruction::AddX(n) => {
                x += *n as i32;
            }
            _ => {}
        }
    }

    scores.iter().map(|(_k, v)| v).sum::<i32>().to_string()
}

struct Handheld {
    x: i32,
    cycle: i32,
    pixels: String,
}
impl Display for Handheld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .chars()
                .chunks(40)
                .into_iter()
                .map(|c| c.collect::<String>())
                .join("\n")
        )
    }
}
impl Handheld {
    pub fn run(self: &mut Self, instructions: Vec<Instruction>) {
        for instr in instructions.iter() {
            self.do_instr(instr);
        }
    }

    fn do_instr(self: &mut Self, instruction: &Instruction) {
        let sprite_pos = (self.x - 1)..=(self.x + 1);
        for i in 0..instruction.cycles() {
            let print_pos = self.cycle + i as i32;
            if sprite_pos.contains(&print_pos) {
                self.pixels.push_str("#");
            } else {
                self.pixels.push_str(".");
            }
        }

        self.cycle = (self.cycle + instruction.cycles() as i32) % 40;
        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(n) => self.x = (self.x + (*n as i32)) % 40,
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let mut handheld = Handheld {
        x: 1,
        cycle: 0,
        pixels: "".to_string(),
    };

    let (_, instructions) = input_parser::parse(input).unwrap();
    handheld.run(instructions);

    handheld.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_works() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "13140");
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(
            process_part2(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
