use crate::day03::{Input};

pub fn solve(input: &Input) -> u32 {
    0
}

fn score_chars(c: char) -> u32 {
    match c { 
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => ((c as u32) - 65) + 27,
        _ => panic!()
    }
}

fn find_badge(s1: &str, s2: &str, s3: &str) -> char {
    'a'
 }