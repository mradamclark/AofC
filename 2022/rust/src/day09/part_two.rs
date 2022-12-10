use std::{collections::HashSet, pin};

use itertools::Itertools;

use crate::day09::{Input, Output};

use super::{Direction, is_tail_attached, follow_knot, move_knot};

pub fn solve(input: &Input) -> Output {
    let mut knots = [(0, 0); 10];
    let mut tail_locations = HashSet::from([knots[knots.len()-1]]);

    for movement in input {
        let mut dir = movement.0;
        for i in 0..movement.1 {
            move_knot(dir, &mut knots[0]);

            // check if tail is attached, ie, tail position is in the 1 space vicinity of the head.
            // if not attached, move following knots. 
            for i in 1..knots.len() {
                let h_index = i-1;
                let t_index = i;
                
                if (!is_tail_attached(knots[h_index], knots[t_index])) {
                    follow_knot(knots[h_index], &mut knots[t_index]);
                }
                else {
                    // if the knot was attached, all other knots will be attached too,
                    // its okay to break out of loop now.
                    break;
                }
            }
            tail_locations.insert(knots[knots.len() - 1]);
        }
    }
    Output::U32(tail_locations.len() as u32)
}