use std::collections::HashSet;

use itertools::Itertools;

use crate::day09::{Input, Output};

use super::{Direction, is_tail_attached, follow_knot, move_knot};

pub fn solve(input: &Input) -> Output {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_locations = HashSet::from([tail]);

    for movement in input {
        let dir = movement.0;
        for i in 0..movement.1 {

            move_knot(dir, &mut head);

            if !is_tail_attached(head, tail) {
                follow_knot(head, &mut tail);
                tail_locations.insert(tail);
            }
        }
    }

    Output::U32(tail_locations.len() as u32)
}
