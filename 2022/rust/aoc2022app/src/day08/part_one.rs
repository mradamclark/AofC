use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut forest = input.clone();
    let rows = forest.len();
    let cols = forest[0].len();

    for i in 0..rows {
        let mut tallest = forest[i][0].0;
        forest[i][0].1 = true;
        for j in 1..cols {
            if forest[i][j].0 > tallest {
                tallest = forest[i][j].0;
                forest[i][j].1 = true;
            }
        }
    }

    for i in (0..rows).rev() {
        forest[i][cols - 1].1 = true;
        let mut tallest = forest[i][cols - 1].0;
        for j in (0..cols).rev() {
            if forest[i][j].0 > tallest {
                tallest = forest[i][j].0;
                forest[i][j].1 = true;
            }
        }
    }

    for c in 0..cols {
        forest[0][c].1 = true;
        let mut tallest = forest[0][c].0;
        for r in 0..rows {
            if forest[r][c].0 > tallest {
                tallest = forest[r][c].0;
                forest[r][c].1 = true;
            }
        }
    }

    for c in (0..cols).rev() {
        forest[rows - 1][c].1 = true;
        let mut tallest = forest[rows - 1][c].0;
        for r in (0..rows).rev() {
            if forest[r][c].0 > tallest {
                tallest = forest[r][c].0;
                forest[r][c].1 = true;
            }
        }
    }

    let mut visible = 0;
    for i in 0..rows {
        for j in 0..cols {
            print!("({},{})", forest[i][j].0, forest[i][j].1 as u8);
            if forest[i][j].1 {
                visible += 1;
            }
        }
        print!("\n");
    }

    Output::U32(visible)
}
