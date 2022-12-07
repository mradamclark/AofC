use std::collections::HashMap;
use nom::character::complete::u32;
use crate::day07::Input;

const INPUT: &str = include_str!("../../../input/day07/input.txt");

pub fn read() -> Input {

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    for line in INPUT.trim().split("\n").map(str::to_string) {
        let cmd = &line[2..];
        if cmd.starts_with("cd") {     
            let parts: (&str, &str) = cmd.split_once(" ").unwrap();
            if parts.1 == ".." {
                path.pop();
            }
            else {
                path.push(parts.1.to_string());
            }
        } else if cmd.starts_with("ls") || line.starts_with("dir") {
        }
        else {
            let file_info: (&str, &str) = line.trim().split_once(" ").unwrap();

            for i in 0..(path.len()) {
                let key = path[0..i+1].join("/").to_string();
                let sz = dir_sizes.get(&key).cloned().unwrap_or(0);
                dir_sizes.insert(key, sz + file_info.0.parse::<u32>().unwrap());
            }
        }
    }

    dir_sizes
 }