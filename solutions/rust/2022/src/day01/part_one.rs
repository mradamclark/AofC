use crate::day01::Input;

pub fn solve(input: &Input) -> u32 {
    input.iter().copied().max().unwrap().into()
}