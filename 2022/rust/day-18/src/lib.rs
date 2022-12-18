use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Lava,
    Air,
    Water,
}

type Grid = [[[Element; 30]; 30]; 30];

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn coord(s: &str) -> IResult<&str, (usize, usize, usize)> {
    let (s, x) = complete::u8(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = complete::u8(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, z) = complete::u8(s)?;

    Ok((s, (x as usize, y as usize, z as usize)))
}

fn parse(s: &str) -> IResult<&str, Grid> {
    let (s, coords) = separated_list1(line_ending, coord)(s)?;

    let mut grid: Grid = [[[Element::Air; 30]; 30]; 30];
    for (x, y, z) in coords {
        grid[x][y][z] = Element::Lava;
    }

    Ok((s, grid))
}

fn count_all_exposed(grid: &Grid, element: Element) -> u32 {
    let neighbours: [(i8, i8, i8); 6] = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut exposed_side = 0;

    for x in 0..30 {
        for y in 0..30 {
            for z in 0..30 {
                if grid[x][y][z] == Element::Lava {
                    for (dx, dy, dz) in neighbours {
                        let ux = x as i8 + dx;
                        let uy = y as i8 + dy;
                        let uz = z as i8 + dz;

                        if ux < 0 || uy < 0 || uz < 0 {
                            exposed_side += 1;
                        } else if ux >= 30 || uy >= 30 || uz >= 30 {
                            exposed_side += 1;
                        } else if grid[ux as usize][uy as usize][uz as usize] == element {
                            exposed_side += 1;
                        }
                    }
                }
            }
        }
    }

    exposed_side
}

fn get_neighbours(g: &Grid, p: Point) -> Vec<Point> {
    let neighbours: [(i8, i8, i8); 6] = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut points: Vec<Point> = Vec::new();

    for (dx, dy, dz) in neighbours {
        let ux = p.x as i8 + dx;
        let uy = p.y as i8 + dy;
        let uz = p.z as i8 + dz;

        if ((0..30).contains(&ux) && (0..30).contains(&uy) && (0..30).contains(&uz))
            && g[ux as usize][uy as usize][uz as usize] == Element::Air
        {
            points.push(Point {
                x: ux as usize,
                y: uy as usize,
                z: uz as usize,
            });
        }
    }

    points
}

pub fn process_part1(input: &str) -> String {
    let (_, grid) = parse(input).unwrap();
    count_all_exposed(&grid, Element::Air).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut grid) = parse(input).unwrap();

    let sp = Point {
        x: 29,
        y: 29,
        z: 29,
    };

    let mut frontier: VecDeque<Point> = VecDeque::new();
    frontier.push_back(sp);

    while !frontier.is_empty() {
        let p = frontier.pop_front().unwrap();
        if grid[p.x][p.y][p.z] == Element::Air {
            grid[p.x][p.y][p.z] = Element::Water;
            let neighbours = get_neighbours(&grid, p);
            for n in neighbours {
                frontier.push_back(n);
            }
        }
    }

    count_all_exposed(&grid, Element::Water).to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "64");
    }

    #[test]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "58");
    }
}
