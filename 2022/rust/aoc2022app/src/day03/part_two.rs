use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let r = input
        .chunks(3)
        .map(|group| find_badge(group.into()))
        .map(|badge| score_badge(badge))
        .sum::<u32>();
    Output::U32(r)
}

fn score_badge(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => ((c as u32) - 65) + 27,
        _ => panic!(),
    }
}

fn find_badge(group: &[&str]) -> char {
    let rucksacks: Vec<Vec<char>> = group.iter().map(|g| g.chars().collect()).collect();
    let mut badge: char = '-';
    for item in &rucksacks[0] {
        if rucksacks[1].contains(&item) && rucksacks[2].contains(&item) {
            return *item;
        } else if rucksacks[2].contains(&item) {
            badge = *item;
        }
    }
    badge
}
