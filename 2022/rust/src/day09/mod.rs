pub mod input;
pub mod part_one;
pub mod part_two;

use itertools::Itertools;

use crate::{Output, Part};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_knot(dir: Direction, head: &mut (i32, i32)) {
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
}

pub fn is_tail_attached(head: (i32, i32), tail: (i32,i32)) -> bool {
    let is_attached =  (head.0 - 1..=head.0 + 1)
        .cartesian_product(head.1 - 1..=head.1 + 1)
        .any(|pos| pos == tail);
    is_attached
}

pub fn follow_knot(leader: (i32,i32), follower: &mut (i32,i32)) {
    let diff = (leader.0 - follower.0, leader.1 - follower.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        follower.0 += diff.0.signum();
        follower.1 += diff.1.signum();
    }
}

pub type Input = Vec<(Direction, u32)>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part_one::solve(&input),
        Part::Two => part_two::solve(&input),
    }
}
