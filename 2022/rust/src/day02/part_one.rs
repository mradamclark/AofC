use crate::day02::{Input, RPS, STATE};

pub fn solve(input: &Input) -> u32 {
    input
        .iter()
        .map(|s| parse_rps_game(*s))
        .map(|g| evalute_game(g.0, g.1))
        .map(|wld| (wld.0 as u32) + (wld.1 as u32))
        .sum()
}

fn parse_rps_game(game: (&str, &str)) -> (RPS, RPS) {
    let a: RPS = match game.0 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => todo!(),
    };
    let b: RPS = match game.1 {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => todo!(),
    };

    (a, b)
}

fn evalute_game(opp: RPS, you: RPS) -> (STATE, RPS) {
    let win_lose_draw = match opp {
        RPS::Rock => match you {
            RPS::Paper => STATE::Win,
            RPS::Scissors => STATE::Lost,
            _ => STATE::Draw,
        },
        RPS::Paper => match you {
            RPS::Rock => STATE::Lost,
            RPS::Scissors => STATE::Win,
            _ => STATE::Draw,
        },
        RPS::Scissors => match you {
            RPS::Rock => STATE::Win,
            RPS::Paper => STATE::Lost,
            _ => STATE::Draw,
        },
    };

    (win_lose_draw, you)
}
