use crate::day05::{CargoStack, CargoStacks, Input, Instruction, Output};

pub fn solve(input: &Input) -> Output {
    let mut cargo_map: CargoStacks = input.0.clone();
    let instructions: Vec<Instruction> = input.1.clone();

    for instr in instructions {
        let mut moving = String::new();
        for i in 0..instr.amount {
            moving.push(cargo_map[instr.from.into()].pop());
        }

        for ch in moving.chars().rev() {
            cargo_map[instr.to.into()].push(ch);
        }
    }

    let mut out = String::new();
    for stack in cargo_map.0.iter_mut() {
        let ch = stack.pop();
        out.push(ch);
    }

    Output::String(out)
}
