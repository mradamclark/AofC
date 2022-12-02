use std::fs;
use std::io::BufReader;

use crate::day02::{Input, RPS};

const INPUT: &str = include_str!("../../../input/day02/input.txt");

pub fn read() -> Input<'static> {
    INPUT
        .trim()
        .split("\n")
        .map(|g| g.split_once(" ").unwrap())
        .collect()
}
