use crate::day07::{Input, Output};
use std::cmp;

pub fn solve(input: &Input) -> Output {
    let max_used:u32 = 70000000 - 30000000;
    let total_used:u32 = *input.get("/").unwrap();
    let amount_to_free:u32 = total_used - max_used;
    
    let mut size_of_dir :u32 = u32::MAX;
    for dir in input {
        if dir.1 >= &amount_to_free {
            size_of_dir = cmp::min(size_of_dir, *dir.1);
        }
    }
    
    Output::U32(size_of_dir)
}

