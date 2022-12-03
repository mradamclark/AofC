use std::fs;
use std::io::BufReader;

use crate::day01::Input;

const INPUT: &str = include_str!("../../../input/01.input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n")
        .flat_map(|l| l.parse::<u32>())
        .collect()
}
