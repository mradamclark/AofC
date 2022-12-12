use std::path;

use itertools::all;
use pathfinding::{directed::dijkstra, prelude::dijkstra};

fn read_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|letters: &str| letters.chars().collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
struct Position(usize, usize);

fn successors(p: &Position, g: &Vec<Vec<char>>) -> Vec<(Position, usize)> {
    let x = p.0 as i8;
    let y = p.1 as i8;
    let current = g[p.0][p.1] as u8;
    let potential: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut moves: Vec<(Position, usize)> = Vec::new();

    for m in potential {
        let nx = x + m.0;
        let ny = y + m.1;

        if nx >= 0 && nx < (g.len() as i8) {
            if ny >= 0 && ny < g[0].len() as i8 {
                let neighbour = g[nx as usize][ny as usize] as u8;
                if neighbour <= current + 1 {
                    moves.push((Position(nx as usize, ny as usize), 1 as usize))
                }
            }
        }
    }

    moves
}

fn find_point(grid: &mut Vec<Vec<char>>, ch: char, replace_with: char) -> Position {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == ch {
                grid[x][y] = replace_with;
                return Position(x, y);
            }
        }
    }
    panic!("not found")
}

pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = read_input(input);

    let start = find_point(&mut grid, 'S', 'a');
    let goal = find_point(&mut grid, 'E', 'z');

    let output = dijkstra(&start, |p| successors(p, &grid), |p| *p == goal);

    output.unwrap().1.to_string()
}

fn find_all_start_points(grid: &mut Vec<Vec<char>>, ch: char, replace_with: char) -> Vec<Position> {
    let mut points: Vec<Position> = Vec::new();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == ch || grid[x][y] == replace_with {
                grid[x][y] = replace_with;
                points.push(Position(x, y));
            }
        }
    }
    points
}

pub fn process_part2(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = read_input(input);

    let start = find_all_start_points(&mut grid, 'S', 'a');
    let goal = find_point(&mut grid, 'E', 'z');

    let mut all_paths: Vec<(Position, usize)> = Vec::new();

    for s in start {
        let output = dijkstra(&s, |p| successors(p, &grid), |p| *p == goal);
        if output != None {
            all_paths.push((s, output.unwrap().1));
        }
    }

    all_paths.sort_by(|a, b| a.1.cmp(&b.1));
    all_paths[0].1.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "29");
    }
}
