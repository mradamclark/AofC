use std::fs;
use std::io::BufReader;

use crate::day01::Input;

const INPUT: &str = include_str!("../../../input/day01/input.txt");

pub fn read() -> Input {
    INPUT.trim()
         .split("\n\n")
         .map(parse_food_item_calories)
         .collect()
}

fn parse_food_item_calories(v: &str) -> u32 {
    v.lines()
     .flat_map(|l| l.parse::<u32>())
     .sum::<u32>()
}