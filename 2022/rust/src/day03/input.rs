use std::fs;
use std::io::BufReader;

use crate::day03::{Input};

const INPUT: &str = include_str!("../../../input/day03/input.txt");

pub fn read() -> Input<'static> {
    INPUT
        .trim()
        .split("\n")
        .map(|l| parse_rucksack(l))
        .collect()
}

fn parse_rucksack(contents: &str) -> (&str, &str) {
    let mid = contents.len() / 2;
    let rucksack: (&str, &str) = contents.split_at(mid);
    rucksack
}
