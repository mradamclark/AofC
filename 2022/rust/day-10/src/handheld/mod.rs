use std::fmt::Display;

use itertools::Itertools;

use crate::parser::Instruction;

pub struct Handheld {
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
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            pixels: "".to_string(),
        }
    }
    pub fn run(self: &mut Self, instructions: Vec<Instruction>) {
        for instr in instructions.iter() {
            self.do_instr(instr);
        }
    }

    fn do_instr(self: &mut Self, instruction: &Instruction) {
        let sprite_pos = (self.x - 1)..=(self.x + 1);
        for i in 0..instruction.cycles() {
            let print_pos = (self.cycle + i as i32) % 40;
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
