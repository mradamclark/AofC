use day_11::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data/input.txt").unwrap();
    println!("AOC2022 Day 11 Part1: {}", process_part1(&file));
}
