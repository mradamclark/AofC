use crate::day07::{Input, Output};
use itertools::Itertools;
use nom::character::complete::u32;
use std::collections::HashMap;

const MAX_SIZE: u32 = 100000;

pub fn solve(input: &Input) -> Output {
    let mut total_sum: u32 = 0;
    for dir in input.keys().sorted() {
        if input[dir] <= MAX_SIZE {
            total_sum += input[dir];
        }
    }

    Output::U32(total_sum)
}
