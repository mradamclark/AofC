use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut v: Vec<u32> = (*input).to_vec();
    v.sort_by(|a, b| b.cmp(a));
    return Output::U32(v[0] + v[1] + v[2]);
}
