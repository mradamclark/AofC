use day_22::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data/input.txt").unwrap();
    println!("AOC2022 Day 22 Part2: {}", process_part2(&file));
}
