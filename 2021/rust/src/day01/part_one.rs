use crate::day01::Input;

pub fn solve(input: &Input) -> u32 {
    let mut increase_cnt = 0;
    let mut prev_depth: u32 = input[0];
    for i in input {
        let next_depth = *i;
        if next_depth > prev_depth {
            increase_cnt += 1;
        }
        prev_depth = next_depth;
    }
    increase_cnt
}
