use itertools::Itertools;
use nom::character::complete::u8;

use crate::day08::Input;

const INPUT: &str = include_str!("../../../../input/day08/input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|i| (i, false, 0))
                .collect()
        })
        .collect()
}
