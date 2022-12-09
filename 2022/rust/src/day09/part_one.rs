use std::collections::HashSet;

use itertools::Itertools;

use crate::day09::{Input, Output};

use super::Direction;

pub fn solve(input: &Input) -> Output {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_locations = HashSet::from([tail]);

    for movement in input {
        let dir = movement.0;
        for i in 0..movement.1 {
            //Move Head
            match dir {
                Direction::Up => {
                    head.1 += 1;
                }
                Direction::Right => {
                    head.0 += 1;
                }
                Direction::Down => {
                    head.1 -= 1;
                }
                Direction::Left => {
                    head.0 -= 1;
                }
            }

            //check if tail is attached, ie, tail position is in the 1 space vicinity of the head.
            let is_attached = ((head.0 - 1)..=(head.0 + 1))
                .cartesian_product((head.1 - 1)..=(head.1 + 1))
                .any(|pos| pos == tail);

            //if tail not attached move, tail.
            if (!is_attached) {
                let mut new_tail = head.clone();
                match dir {
                    Direction::Up => {
                        new_tail.1 -= 1;
                    }
                    Direction::Right => {
                        new_tail.0 -= 1;
                    }
                    Direction::Down => {
                        new_tail.1 += 1;
                    }
                    Direction::Left => {
                        new_tail.0 += 1;
                    }
                }
                tail_locations.insert(new_tail);
                tail = new_tail;
            }
        }
    }

    Output::U32(tail_locations.len() as u32)
}
