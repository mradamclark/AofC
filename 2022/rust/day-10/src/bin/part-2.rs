use day_10::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data/input.txt").unwrap();
    println!("{}", process_part2(&file));
}
