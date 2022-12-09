use std::{collections::HashSet, pin};

use itertools::Itertools;

use crate::day09::{Input, Output};

use super::Direction;

pub fn solve(input: &Input) -> Output {
    let mut knots = [(0, 0); 10];
    let mut tail_locations = HashSet::from([knots[knots.len()-1]]);

    for movement in input {
        println!("{:?}", movement);
        let mut dir = movement.0;
        for i in 0..movement.1 {
            //Move Head
            match dir {
                Direction::Up => { 
                    knots[0].1 += 1;
                }
                Direction::Right => {
                    knots[0].0 += 1;
                }
                Direction::Down => {
                    knots[0].1 -= 1;
                }
                Direction::Left => {
                    knots[0].0 -= 1;
                }
            }

            //check if tail is attached, ie, tail position is in the 1 space vicinity of the head.
            // println!("{:?} - {:?}", knots, tail_locations);
            for i in 1..knots.len() {
                let h_index = i-1;
                let t_index = i;
                
                // println!("h{:?}, t{:?}", knots[h_index], knots[t_index]);

                let x_range = ((knots[h_index].0) - 1..=(knots[h_index].0 + 1));
                let y_range = ((knots[h_index]).1 - 1..=(knots[h_index].1) + 1); 
                let cartesian_plane =  x_range.cartesian_product(y_range).collect_vec();
                let is_attached = cartesian_plane.clone().into_iter().any(|pos| pos == knots[t_index]);
            
                if (!is_attached) {
                    
                    if (knots[h_index].0 == knots[t_index].0) {
                        if (knots[h_index].1 > knots[t_index].1) {
                            knots[t_index].1 += 1;
                        }
                        else {
                            knots[t_index].1 -= 1;
                        }
                    }
                    else if (knots[h_index].1 == knots[t_index].1) {
                        if (knots[h_index].0 > knots[t_index].0) {
                            knots[t_index].0 += 1;
                        }
                        else {
                            knots[t_index].0 -= 1;
                        }
                    }
                    else {
                        //diagonal stuff
                   
                        if (knots[h_index].0 > knots[t_index].0) {
                            knots[t_index].0 += 1;
                            if (knots[h_index].1 > knots[t_index].1) {
                                knots[t_index].1 += 1;
                            }
                            else {
                                knots[t_index].1 -= 1;
                            }
                        }
                        else if (knots[h_index].0 < knots[t_index].0) {
                            knots[t_index].0 -= 1;
                            if (knots[h_index].1 > knots[t_index].1) {
                                knots[t_index].1 += 1
                            }
                            else {
                                knots[t_index].1 -= 1;
                            }
                        }
                    }
                 }    
                // println!("{:?}", knots);  
            }
            tail_locations.insert(knots[knots.len() - 1]);

        }
    }
    Output::U32(tail_locations.len() as u32)
}
