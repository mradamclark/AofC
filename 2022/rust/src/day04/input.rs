use std::fs;
use std::io::BufReader;

use crate::day04::Input;

const INPUT: &str = include_str!("../../../input/day04/input.txt");

pub fn read() -> Input<'static> {
    INPUT
        .trim()
        .split("\n")
        .collect()
}
