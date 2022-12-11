use std::collections::HashMap;
pub mod handheld;
pub mod parser;

use handheld::Handheld;

use crate::parser::Instruction;

pub fn process_part1(input: &str) -> String {
    let mut scores: HashMap<u32, i32> = HashMap::new();
    let watched_cycles = [20, 60, 100, 140, 180, 220];
    let (_, instructions) = parser::parse(input).unwrap();

    let mut cycle = 0;
    let mut x: i32 = 1;

    for instr in instructions.iter() {
        if watched_cycles.contains(&(cycle + 1)) {
            scores.insert(cycle + 1, x * (cycle as i32 + 1));
        }

        if watched_cycles.contains(&(cycle + 2)) {
            scores.insert(cycle + 2, x * (cycle as i32 + 2));
        }

        cycle += instr.cycles() as u32;
        match instr {
            Instruction::AddX(n) => {
                x += *n as i32;
            }
            _ => {}
        }
    }

    scores.iter().map(|(_k, v)| v).sum::<i32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut handheld = Handheld::new();

    let (_, instructions) = parser::parse(input).unwrap();
    handheld.run(instructions);

    handheld.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1_works() {
        let input = fs::read_to_string("data/test.txt").unwrap();
        let result = crate::process_part1(&input);
        assert_eq!(result, "13140");
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./data/test.txt").unwrap();
        assert_eq!(
            crate::process_part2(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
