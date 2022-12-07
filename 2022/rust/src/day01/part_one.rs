use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::U32(input.iter().copied().max().unwrap().into())
}
