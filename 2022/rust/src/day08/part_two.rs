use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut forest = input.clone();
    let rows = forest.len();
    let cols = forest[0].len();
    print!("\n");
    for i in 0..rows {
        for j in 0..cols {
            let height = forest[i][j].0;
            // println!("pos: {},{}",i,j);

            let mut up_vd = 0;
            if i > 0 {
                for r in (0..i).rev() {
                    if forest[r][j].0 < height {
                        up_vd += 1;
                    } else {
                        up_vd += 1;
                        break;
                    }
                }
            }

            let mut left_vd = 0;
            if j > 0 {
                for c in (0..j).rev() {
                    if forest[i][c].0 < height {
                        left_vd += 1;
                    } else {
                        left_vd += 1;
                        break;
                    }
                }
            }

            let mut down_vd = 0;
            if i < rows - 1 {
                for r in (i + 1..rows) {
                    if forest[r][j].0 < height {
                        down_vd += 1;
                    } else {
                        down_vd += 1;
                        break;
                    }
                }
            }

            let mut right_vd = 0;
            if j < cols - 1 {
                for c in (j + 1..cols) {
                    // println!("checking [{}][{}](h:{})", i, c, forest[i][c].0);
                    if forest[i][c].0 < height {
                        right_vd += 1;
                    } else {
                        right_vd += 1;
                        break;
                    }
                }
            }

            forest[i][j].2 = up_vd * left_vd * down_vd * right_vd;
            // println!("pos[{}][{}]({}) - {}*{}*{}*{} = {}", i, j, forest[i][j].0, up_vd, left_vd, down_vd, right_vd, forest[i][j].2);
        }
    }

    let mut score = 0;
    for i in 0..rows {
        for j in 0..cols {
            print!("({},{})", forest[i][j].0, forest[i][j].2);
            if forest[i][j].2 > score {
                score = forest[i][j].2
            }
        }
        print!("\n")
    }

    Output::U32(score)
}
