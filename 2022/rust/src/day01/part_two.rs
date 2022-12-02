use crate::day01::Input;

pub fn solve(input: &Input) -> u32 {
    let mut v: Vec<u32> = (*input).to_vec();
    v.sort_by(|a, b| b.cmp(a));
    return v[0] + v[1] + v[2];
}
