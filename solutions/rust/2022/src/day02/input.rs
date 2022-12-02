use std::fs;
use std::io::BufReader;

use crate::day02::{Input, RPS};

const INPUT: &str = include_str!("../../input/day02/input.txt");

pub fn read() -> Input<'static> {
    INPUT.trim()
         .split("\n")
         .map(|g| g.split_once(" ").unwrap())
         .collect()
}

// fn parse_rps_game(v: &str) -> (RPS, RPS) {
//     let game: (&str,&str) = v.split_once(" ").unwrap();
//     let a: RPS = match game.0 { 
//         "A" => RPS::Rock,
//         "B" => RPS::Paper,
//         "C" => RPS::Scissors,
//         _ => todo!(),
//      };
//     let b: RPS = match game.1 {
//         "X" => RPS::Rock,
//         "Y" => RPS::Paper,
//         "Z" => RPS::Scissors,
//         _ => todo!(),
//      };

//     (a, b)
// }