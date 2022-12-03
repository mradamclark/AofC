use crate::day03::{Input};
use std::collections::HashSet;

pub fn solve(input: &Input) -> u32 {
    input.iter()
        .map(|contents| contents.split_at(contents.len()/2))
        .map(|rucksack| common_chars(rucksack.0,rucksack.1))
        .map(|h| score_chars(h))
        .sum::<u32>()
}

fn score_chars(shared_items: HashSet<char>) -> u32 {
    let scores: Vec<u32> = shared_items.iter()
                                        .map(|i| match *i { 'a'..='z' => (*i as u32) - 96,
                                                            'A'..='Z' => ((*i as u32) - 65) + 27,
                                                            _ => panic!()
                                        })
                                        .collect();
    scores.iter().sum()
}

fn common_chars(s1: &str, s2: &str) -> HashSet<char> {
    let mut shared: HashSet<char> = HashSet::new(); 

    for c in s1.chars() {
        if s2.contains(c) {
            shared.insert(c);
        }
    }
    shared
 }