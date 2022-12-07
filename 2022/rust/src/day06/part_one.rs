use crate::day06::{Input, Output};

pub fn solve(input: &Input) -> Output {

    for line in input.trim().split("\n") {
        for i in 0..(line.len()-4) {
            let buffer = &line[i..(i+4)];
            if detect_unique(buffer) {
                return Output::U32((i+4).try_into().unwrap());
            }
        }
    }
    panic!("crap, found nothing");
}

fn detect_unique(buf: &str) -> bool {
    let a: Vec<char> = buf.chars().collect();
    let mut b = a.clone();
    b.sort();
    b.dedup();
    a.len() == b.len()
}

