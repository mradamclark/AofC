use crate::day01::Input;

pub fn solve(input: &Input) -> u32 {
    let mut increase_cnt = 0;
    let mut prev_depth: u32  = input[0] + input[1] + input[2];

    for i in 1..=input.len() - 3 {
        let next_depth = input[i] + input[i+1] + input[i+2];
        if prev_depth < next_depth {
            increase_cnt += 1;
        }
        prev_depth = next_depth;
    }
    increase_cnt
}

fn print_vector(v: &Vec<u32>) {
    for n in v {
        println!("{}",n);
    }
}