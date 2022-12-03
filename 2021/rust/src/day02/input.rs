use std::fs;
use std::io::BufReader;

use crate::day02::{Input, Direction, SubCommand};

const INPUT: &str = include_str!("../../../input/02.test.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n")
        .map(|l| l.split_once(" ").unwrap())
        .map(|p| SubCommand {
                    direction: p.0.parse().unwrap_or_else(|error| { panic!("Problem creating the enum: {:?}", error); }), 
                    amount: p.1.parse::<u32>().unwrap(),
                })
        .collect()
}
