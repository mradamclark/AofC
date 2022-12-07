use crate::day05::{CargoStack, Input, Output, CargoStacks, Instruction};

pub fn solve(input: &Input) -> Output {

    let mut cargo_map: CargoStacks = input.0.clone();
    let instructions: Vec<Instruction> = input.1.clone();

    println!("{}", cargo_map);

    for i in instructions {
        println!("{}",i);
    }

    Output::String("WTF".to_string())
}

