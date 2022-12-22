use std::ops::Index;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{line_ending, one_of},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    *,
};
use tracing::instrument;

fn intructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(
        many1(alt((
            complete::u32.map(|n| Instruction::Move(n as usize)),
            complete::char('R').map(|_c| Instruction::Right),
            complete::char('L').map(|_c| Instruction::Left),
        ))),
        alt((line_ending, eof)),
    )(s)
}

fn grid(s: &str) -> IResult<&str, Map> {
    let (s, tiles) = separated_list1(line_ending, many1(one_of(" .#")))(s)?;
    let rows = tiles.len();
    let longest = tiles.iter().map(|t| t.len()).max().unwrap();

    let mut ntiles = Vec::<char>::new();
    for t in tiles {
        let mut nt = t.clone();
        nt.resize(longest, ' ');
        ntiles.append(&mut nt);
    }

    Ok((
        s,
        Map {
            data: ntiles,
            rmax: rows as i64,
            cmax: longest as i64,
        },
    ))
}

fn parse(s: &str) -> IResult<&str, (Map, Vec<Instruction>)> {
    separated_pair(grid, tag("\n\n"), intructions)(s)
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

struct Map {
    data: Vec<char>,
    rmax: i64,
    cmax: i64,
}
impl Map {
    pub fn step(
        &self,
        (mut row, mut col, dir): (usize, usize, Direction),
        steps: usize,
    ) -> (usize, usize) {
        println!("Move {dir:?} {steps:?} steps");

        let forward = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        print!("\t step onto ({row:?},{col:?})->");
        for _s in 1..=steps {
            let delta = forward[dir as usize];
            let (nr, nc) = self.idx_region_wrap((row, col), delta, dir);
            print!("({row:?},{col:?})->");
            if self[(nr, nc)] == '#' {
                break;
            }
            (row, col) = (nr, nc);
        }
        print!("({row:?},{col:?})\n");
        // print!(" stopping at ({row:?},{col:?}).\n");
        (row, col)
    }

    fn idx_wrap(&self, (r, c): (usize, usize), (dr, dc): (i64, i64)) -> (usize, usize) {
        let nr = (((r as i64 + dr) % self.rmax) + self.rmax) % self.rmax;
        let nc = (((c as i64 + dc) % self.cmax) + self.cmax) % self.cmax;

        (nr as usize, nc as usize)
    }

    fn idx_region_wrap(
        &self,
        (row, col): (usize, usize),
        (dr, dc): (i64, i64),
        dir: Direction,
    ) -> (usize, usize) {
        let mut nr = ((((row as i64 + dr) % self.rmax) + self.rmax) % self.rmax) as usize;
        let mut nc = ((((col as i64 + dc) % self.cmax) + self.cmax) % self.cmax) as usize;
        // println!("{nr:?}, {nc:?}");
        let reverse = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        if self[(nr, nc)] == ' ' {
            // println!("{nr:?}, {nc:?}");
            loop {
                let pr = nr;
                let pc = nc;
                let delta = reverse[dir as usize];
                (nr, nc) = self.idx_wrap((nr, nc), delta);
                // println!("{nr:?}, {nc:?}");
                if self[(nr, nc)] == ' ' {
                    nr = pr;
                    nc = pc;
                    break;
                }
            }
        }
        (nr, nc)
    }
}
impl Index<(usize, usize)> for Map {
    type Output = char;
    fn index(&self, idx: (usize, usize)) -> &char {
        // println!("\t\t[{:?},{:?}]", idx.0, idx.1);
        &self.data[idx.0 * self.cmax as usize + idx.1]
    }
}

fn left(dir: &Direction) -> Direction {
    match *dir {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
    }
}

fn right(dir: &Direction) -> Direction {
    match *dir {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

#[instrument]
pub fn process_part1(input: &str) -> String {
    let (_s, (map, instructions)) = parse(input).unwrap();

    let (mut row, mut col) = (0, 0);
    let mut dir = Direction::Right;

    while map[(row, col)] != '.' {
        col += 1;
    }

    println!(
        "start: ({row:?},{col:?}), max: ({:?},{:?})",
        map.rmax, map.cmax
    );
    for instr in instructions.iter() {
        match instr {
            Instruction::Move(n) => {
                (row, col) = map.step((row, col, dir), *n);
            }
            Instruction::Left => {
                println!("Turn Left");
                dir = left(&dir);
            }
            Instruction::Right => {
                println!("Turn Right");
                dir = right(&dir);
            }
        }
    }

    // row and col are not zero based in the scoring, but i used 0 based index in the map.
    ((1000 * (row + 1)) + (4 * (col + 1)) + (dir as usize)).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_s, (_map, _instructions)) = parse(input).unwrap();
    0.to_string()
}
#[cfg(test)]
mod tests {
    use crate::{parse, Direction, Map};

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        tracing_subscriber::fmt::init();
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "6032");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "301");
    }

    #[test]
    fn test_map_idx_wrap() {
        let map: Map = Map {
            data: Vec::<char>::new(),
            rmax: 10,
            cmax: 10,
        };
        let row: usize = 9;
        let col: usize = 9;

        let (r, c) = map.idx_wrap((row, col), (1, 0));
        assert_eq!((r, c), (0, 9));

        let (r, c) = map.idx_wrap((row, col), (0, 1));
        assert_eq!((r, c), (9, 0));

        let row: usize = 0;
        let col: usize = 0;
        let (r, c) = map.idx_wrap((row, col), (-1, 0));
        assert_eq!((r, c), (9, 0));

        let (r, c) = map.idx_wrap((row, col), (0, -1));
        assert_eq!((r, c), (0, 9));
    }

    #[test]
    fn test_map_idx_region_wrap_with_with_space_on_outside() {
        let map: Map = Map {
            data: vec![
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                '.', '.', '.', ' ', ' ', ' ', ' ', '.', '#', '.', ' ', ' ', ' ', ' ', '.', '.',
                '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ',
            ],
            rmax: 7,
            cmax: 7,
        };

        let row: usize = 3;
        let col: usize = 4;
        let (nr, nc) = map.idx_region_wrap((row, col), (0, 1), Direction::Right);
        assert_eq!((nr, nc), (3, 2));

        let row: usize = 3;
        let col: usize = 2;
        let (nr, nc) = map.idx_region_wrap((row, col), (0, -1), Direction::Left);
        assert_eq!((nr, nc), (3, 4));

        let row: usize = 4;
        let col: usize = 3;
        let (nr, nc) = map.idx_region_wrap((row, col), (1, 0), Direction::Down);
        assert_eq!((nr, nc), (2, 3));

        let row: usize = 2;
        let col: usize = 3;
        let (nr, nc) = map.idx_region_wrap((row, col), (-1, 0), Direction::Up);
        assert_eq!((nr, nc), (4, 3));
    }

    #[test]
    fn test_map_idx_region_wrap_with_with_space_between_regions_in_path() {
        let map: Map = Map {
            data: vec![
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                '.', '.', '.', ' ', ' ', ' ', ' ', '.', ' ', '.', ' ', ' ', ' ', ' ', '.', '.',
                '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ',
            ],
            rmax: 7,
            cmax: 7,
        };

        let row: usize = 3;
        let col: usize = 4;
        let (nr, nc) = map.idx_region_wrap((row, col), (0, 1), Direction::Right);
        assert_eq!((nr, nc), (row, 4));

        let row: usize = 3;
        let col: usize = 2;
        let (nr, nc) = map.idx_region_wrap((row, col), (0, -1), Direction::Left);
        assert_eq!((nr, nc), (row, 2));

        let row: usize = 4;
        let col: usize = 3;
        let (nr, nc) = map.idx_region_wrap((row, col), (1, 0), Direction::Down);
        assert_eq!((nr, nc), (4, col));

        let row: usize = 2;
        let col: usize = 3;
        let (nr, nc) = map.idx_region_wrap((row, col), (-1, 0), Direction::Up);
        assert_eq!((nr, nc), (2, col));
    }

    #[test]
    fn test_map_idx_region_wrap_test_input_example() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.   
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        let (_s, (map, _instructions)) = parse(input).unwrap();

        let position = (6, 11);
        let expected = (6, 0);

        let (r, c) = map.idx_region_wrap(position, (0, 1), Direction::Right);
        assert_eq!((r, c), expected);
    }
}
