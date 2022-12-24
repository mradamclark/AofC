use itertools::Itertools;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    *,
};

use std::collections::{BTreeMap, HashSet};

type Location = (i32, i32);
const SE_DELTA: Location = (1, 1);
const S_DELTA: Location = (1, 0);
const SW_DELTA: Location = (1, -1);
const E_DELTA: Location = (0, 1);
const W_DELTA: Location = (0, -1);
const NE_DELTA: Location = (-1, 1);
const N_DELTA: Location = (-1, 0);
const NW_DELTA: Location = (-1, -1);

const NEIGHBOURS: [Location; 8] = [
    N_DELTA,
    NW_DELTA,
    NE_DELTA,
    W_DELTA,
    E_DELTA,
    SW_DELTA,
    S_DELTA,
    SE_DELTA,
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}
impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn get_step_delta(&self) -> Location {
        match self {
            Direction::North => N_DELTA,
            Direction::South => S_DELTA,
            Direction::West => W_DELTA,
            Direction::East => E_DELTA,
        }
    }
    fn get_frontier(&self, position: &Location) -> [Location; 3] {
        match self {
            Direction::North => [
                (position.0 + NW_DELTA.0, position.1 + NW_DELTA.1),
                (position.0 + N_DELTA.0, position.1 + N_DELTA.1),
                (position.0 + NE_DELTA.0, position.1 + NE_DELTA.1),
            ],
            Direction::South => [
                (position.0 + SW_DELTA.0, position.1 + SW_DELTA.1),
                (position.0 + S_DELTA.0, position.1 + S_DELTA.1),
                (position.0 + SE_DELTA.0, position.1 + SE_DELTA.1),
            ],
            Direction::West => [
                (position.0 + NW_DELTA.0, position.1 + NW_DELTA.1),
                (position.0 + W_DELTA.0, position.1 + W_DELTA.1),
                (position.0 + SW_DELTA.0, position.1 + SW_DELTA.1),
            ],
            Direction::East => [
                (position.0 + NE_DELTA.0, position.1 + NE_DELTA.1),
                (position.0 + E_DELTA.0, position.1 + E_DELTA.1),
                (position.0 + SE_DELTA.0, position.1 + SE_DELTA.1),
            ],
        }
    }
}

fn parse(s: &str) -> IResult<&str, Field> {
    let (s, items) = separated_list1(line_ending, many1(one_of(".#e")))(s)?;

    Ok((
        s,
        Field {
            data: items
                .into_iter()
                .enumerate()
                .flat_map(|(r, v)| {
                    v.into_iter()
                        .enumerate()
                        .filter(|(_, ch)| *ch != '.')
                        .map(move |(c, _)| ((r as i32, c as i32), None))
                })
                .collect(),
        },
    ))
}

struct Field {
    data: BTreeMap<Location, Option<Direction>>,
}
impl Field {
    fn score(&self) -> u32 {
        let rmax = self.data.keys().map(|(x, _)| x).max().unwrap();
        let rmin = self.data.keys().map(|(x, _)| x).min().unwrap();

        let cmax = self.data.keys().map(|(_, y)| y).max().unwrap();
        let cmin = self.data.keys().map(|(_, y)| y).min().unwrap();

        // dbg!(rmax, rmin, cmax, cmin);

        let r = ((rmax - rmin) + 1) as u32;
        let c = ((cmax - cmin) + 1) as u32;

        // dbg!(r, c, self.data.len());
        (r * c) - self.data.len() as u32
    }

    fn do_move(&mut self, loc: &Location, goto: &Location, direction: Direction) -> bool {
        if self.data.contains_key(goto) && loc != goto {
            // print!("goto already there");
            return false;
        }

        self.data.remove(loc).unwrap();
        self.data.insert(goto.clone(), Some(direction));
        return true;
    }

    fn has_neighours(&self, elf_loc: &Location) -> bool {
        NEIGHBOURS.iter().any(|delta| {
            self.data
                .keys()
                .contains(&(elf_loc.0 + delta.0, elf_loc.1 + delta.1))
        })
    }

    fn try_move(&self, loc: &Location, direction: &Direction) -> Option<(Location, Direction)> {
        let delta = direction.get_step_delta();
        let frontier = direction.get_frontier(&loc);

        if frontier.iter().any(|f| self.data.keys().contains(f)) {
            return None;
        }

        Some(((loc.0 + delta.0, loc.1 + delta.1), *direction))
    }

    fn print(&self, padding: i32) {
        let rmax = self.data.keys().map(|(x, _)| x).max().unwrap();
        let rmin = self.data.keys().map(|(x, _)| x).min().unwrap();

        let cmax = self.data.keys().map(|(_, y)| y).max().unwrap();
        let cmin = self.data.keys().map(|(_, y)| y).min().unwrap();

        println!("");
        for r in (*rmin - padding)..=(*rmax + padding) {
            for c in (*cmin - padding)..=(*cmax + padding) {
                if self.data.contains_key(&(r, c)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn print_state(&self, padding: i32) {
        println!("Current State:");
        self.data.iter().for_each(|(elf, direction)| {
            println!(" elf @ {:?}, d: {:?}", elf, direction);
        });
        println!("");
    }
}

fn get_proprosed_move(
    elf_loc: &Location,
    last_direction: Option<Direction>,
    field: &Field,
) -> Option<(Location, Location, Direction)> {
    // println!("--------------");
    // dbg!(elf_loc);
    if !field.has_neighours(elf_loc) {
        //Don't need to move when i have no neighbours.
        return None;
    } else {
        let mut next_direction = Direction::North;
        if last_direction != None {
            next_direction = last_direction.unwrap();
            next_direction = next_direction.next();
        }
        let first_direction_this_round = next_direction;

        let mut cnt = 1;
        loop {
            let proposed_move = field.try_move(elf_loc, &next_direction);
            // print!("{:?}, {:?} -> ", elf_loc, &next_direction);
            // dbg!(proposed_move);
            if proposed_move != None {
                let proposed_move = proposed_move.unwrap();
                // print!("{:?}\n", proposed_move.0);
                return Some((*elf_loc, proposed_move.0, first_direction_this_round));
            } else {
                // print!("None\n");
            }

            if cnt >= 4 {
                break;
            }

            cnt += 1;
            next_direction = next_direction.next();
        }

        return Some((*elf_loc, *elf_loc, first_direction_this_round));
    }
}

fn update_common_move_to_stay_move(
    items: &Vec<(Location, Location, Direction)>,
) -> Vec<(Location, Location, Direction)> {
    let uncommon: HashSet<Location> = items
        .into_iter()
        .map(|(elf, dest, next_direction)| (*dest, (*elf, *dest, next_direction)))
        .into_group_map()
        .iter()
        .filter(|(_, g)| g.len() <= 1)
        .map(|(dest, _)| *dest)
        .collect();

    // dbg!(&uncommon);

    items
        .into_iter()
        .map(|(elf, dest, direction)| {
            if uncommon.contains(dest) {
                (*elf, *dest, *direction)
            } else {
                (*elf, *elf, *direction)
            }
        })
        .collect()
}

pub fn process_part1(input: &str, turns: u32) -> String {
    let (_, mut field) = parse(input).unwrap();

    field.print(0);
    for round in 1..=3 {
        field.print_state(0);
        let mut proposed_moves: Vec<(Location, Location, Direction)> = field
            .data
            .iter()
            .filter_map(|(elf_loc, last_direction)| {
                get_proprosed_move(&elf_loc, *last_direction, &field)
            })
            .collect();

        println!("---Proposed Moves--------");
        proposed_moves = update_common_move_to_stay_move(&proposed_moves);
        print_proposed_moves(&proposed_moves);

        proposed_moves.drain(0..).for_each(|(elf, loc, direction)| {
            field.do_move(&elf, &loc, direction);
        });

        println!("");
        field.print(0);
        println!("\n== End of Round {round:?} ==");
    }
    // field.print(0);
    field.score().to_string()
}

fn print_proposed_moves(pmoves: &Vec<(Location, Location, Direction)>) {
    for m in pmoves {
        println!(
            "{:?} -> {:?}, {:?}",
            (m.0 .0, m.0 .1),
            (m.1 .0, m.1 .1),
            m.2
        );
    }
}

pub fn process_part2(_input: &str) -> String {
    0.to_string()
}
#[cfg(test)]
mod tests {
    use crate::{get_proprosed_move, parse, Direction, Location};

    const SE_DELTA: Location = (1, 1);
    const S_DELTA: Location = (1, 0);
    const SW_DELTA: Location = (1, -1);
    const E_DELTA: Location = (0, 1);
    const W_DELTA: Location = (0, -1);
    const NE_DELTA: Location = (-1, 1);
    const N_DELTA: Location = (-1, 0);
    const NW_DELTA: Location = (-1, -1);

    const AOC_TEST_INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    const AOC_SMALL_TEST_INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    #[test]
    fn part_1_small() {
        let result = crate::process_part1(AOC_SMALL_TEST_INPUT, 3);
        assert_eq!(result, "25");
    }

    #[test]
    fn part_1_works() {
        let result = crate::process_part1(AOC_TEST_INPUT, 10);
        assert_eq!(result, "110");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        let result = crate::process_part2(AOC_TEST_INPUT);
        assert_eq!(result, "5031");
    }

    macro_rules! get_proposed_move_tests {
        ($($name:ident: $input:expr, $elf_location:expr, $last_direction:expr, $expected:expr)*) => {
        $(
            #[test]
            fn $name() {
                let (_, field) = parse($input).unwrap();

                let proposed_move = get_proprosed_move(&$elf_location, $last_direction, &field);
                assert_eq!($expected, proposed_move);
            }
        )*
        }
    }

    get_proposed_move_tests! {
        get_proposed_move_with_nothing_blocking: "...\n.e.\n...", &(1,1), None, None
        propose_move_with_north_blocked_move_south: ".#.\n.e.\n...", &(1, 1), None, Some(((1,1), (2,1), Direction::South))
        propose_move_with_ne_blocked_move_south: "..#\n.e.\n...", &(1, 1), None, Some(((1,1),(2,1), Direction::South))
        propose_move_with_nw_blocked_move_south: "#..\n.e.\n...", &(1, 1),  None, Some(((1,1),(2,1), Direction::South))

        propose_move_with_n_s_blocked_move_west: ".#.\n.e.\n.#.", &(1, 1),  None, Some(((1,1),(1,0), Direction::West))
        propose_move_with_n_s_w_blocked_move_east: ".#.\n#e.\n.#.", &(1, 1), None, Some(((1,1),(1,2), Direction::East))

        propose_move_with_with_w_blocking_and_last_move_was_north: "...\n#e.\n...", &(1,1), Some(Direction::North), Some(((1,1),(2,1), Direction::South))
    }

    macro_rules! test_field_has_neighbours {
        ($($name:ident: $input:expr, $elf_loc:expr, $expected:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let (_, field) = parse($input).unwrap();
                    dbg!($input);
                    dbg!(&field.data);
                    assert_eq!(field.has_neighours($elf_loc), $expected)
                }
            )*
        }
    }
    test_field_has_neighbours! {
        test_field_has_neighours_no_neighbours: "...\n.e.\n...", &(1,1), false
         test_field_has_neighbours_ne_neighbour: "..#\n.e.\n...", &(1,1), true
         test_field_has_neighbours_n_neighbour: ".#.\n.e.\n...", &(1,1), true
         test_field_has_neighbours_nw_neighbour: "#..\n.e.\n...", &(1,1), true
         test_field_has_neighbours_w_neighbour: "...\n#e.\n...", &(1,1), true
         test_field_has_neighbours_e_neighbour: "...\n.e#\n...", &(1,1), true
         test_field_has_neighbours_sw_neighbour: "...\n.e.\n#..", &(1,1), true
         test_field_has_neighbours_s_neighbour: "...\n.e.\n.#.", &(1,1), true
         test_field_has_neighbours_se_neighbour: "...\n.e.\n..#", &(1,1), true
    }

    #[test]
    fn test_direction_next() {
        assert_eq!(Direction::North.next(), Direction::South);
        assert_eq!(Direction::South.next(), Direction::West);
        assert_eq!(Direction::West.next(), Direction::East);
        assert_eq!(Direction::East.next(), Direction::North);
    }

    #[test]
    fn test_direction_get_step_delta() {
        assert_eq!(Direction::North.get_step_delta(), N_DELTA);
        assert_eq!(Direction::South.get_step_delta(), S_DELTA);
        assert_eq!(Direction::West.get_step_delta(), W_DELTA);
        assert_eq!(Direction::East.get_step_delta(), E_DELTA);
    }

    macro_rules! test_try_move {
        ($($name:ident: $input:expr, $elf_loc:expr, $direction:expr, $expected:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let (_, field) = parse($input).unwrap();
                    let elf_move = field.try_move($elf_loc, $direction);
                    assert_eq!(elf_move, $expected)
                }
            )*
        }
    }
    test_try_move! {
        try_move_with_empty_frontier: "...\n.e.", &(1,1), &Direction::North, Some(((0,1), Direction::North))
        try_move_with_left_element_blocked_in_frontier: "#..\n.e.", &(1,1), &Direction::North, None
        try_move_with_right_element_blocked_in_frontier: "..#\n.e.", &(1,1), &Direction::North, None
        try_move_with_center_element_blocked_in_frontier: ".#.\n.e.", &(1,1), &Direction::North, None
        try_move_with_all_blocked_in_frontier: "###...", &(1,1), &Direction::North, None
    }

    #[test]
    fn test_direction_get_frontier() {
        let position: Location = (1, 1);

        let expected: [Location; 3] = [
            (position.0 + NW_DELTA.0, position.1 + NW_DELTA.1),
            (position.0 + N_DELTA.0, position.1 + N_DELTA.1),
            (position.0 + NE_DELTA.0, position.1 + NE_DELTA.1),
        ];
        assert_eq!(expected, Direction::North.get_frontier(&position));
        let expected: [Location; 3] = [
            (position.0 + NW_DELTA.0, position.1 + NW_DELTA.1),
            (position.0 + W_DELTA.0, position.1 + W_DELTA.1),
            (position.0 + SW_DELTA.0, position.1 + SW_DELTA.1),
        ];
        assert_eq!(expected, Direction::West.get_frontier(&position));

        let expected: [Location; 3] = [
            (position.0 + SW_DELTA.0, position.1 + SW_DELTA.1),
            (position.0 + S_DELTA.0, position.1 + S_DELTA.1),
            (position.0 + SE_DELTA.0, position.1 + SE_DELTA.1),
        ];
        assert_eq!(expected, Direction::South.get_frontier(&position));

        let expected: [Location; 3] = [
            (position.0 + NE_DELTA.0, position.1 + NE_DELTA.1),
            (position.0 + E_DELTA.0, position.1 + E_DELTA.1),
            (position.0 + SE_DELTA.0, position.1 + SE_DELTA.1),
        ];
        assert_eq!(expected, Direction::East.get_frontier(&position));
    }
}
