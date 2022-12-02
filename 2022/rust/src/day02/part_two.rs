use crate::day02::{Input, RPS, STATE};

pub fn solve(input: &Input) -> u32 {
    input
        .iter()
        .map(|s| parse_rps_game(*s))
        .map(|g| evalute_game(g.0, g.1))
        .map(|wld| (wld.0 as u32) + (wld.1 as u32))
        .sum()
}

fn parse_rps_game(game: (&str, &str)) -> (RPS, STATE) {
    let a: RPS = match game.0 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let b: STATE = match game.1 {
        "X" => STATE::Lost,
        "Y" => STATE::Draw,
        "Z" => STATE::Win,
        _ => panic!(),
    };

    (a, b)
}

fn evalute_game(opp: RPS, wld: STATE) -> (STATE, RPS) {
    let what = match opp {
        RPS::Rock => match wld {
            STATE::Win => RPS::Paper,
            STATE::Lost => RPS::Scissors,
            _ => RPS::Rock,
        },
        RPS::Paper => match wld {
            STATE::Win => RPS::Scissors,
            STATE::Lost => RPS::Rock,
            _ => RPS::Paper,
        },
        RPS::Scissors => match wld {
            STATE::Win => RPS::Rock,
            STATE::Lost => RPS::Paper,
            _ => RPS::Scissors,
        },
    };

    (wld, what)
}
