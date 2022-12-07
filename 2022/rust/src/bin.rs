use aoc2022lib::*;
use clap::Parser;

#[derive(Parser, Debug)]

struct Args {
    #[arg(default_value_t = 0)]
    day: u8,
}

fn main() {
    let day = Args::parse().day;

    let fn_to_exec = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        _ => panic!("Day {} doesn't exist in AofC calendar", day),
    };

    let answer_one = fn_to_exec(Part::One);
    let answer_two = fn_to_exec(Part::Two);

    println!("");
    println!("AOC2022::Day {} Solution", day);
    println!("    Part One: {}", answer_one);
    println!("    Part Two: {}", answer_two);
}
