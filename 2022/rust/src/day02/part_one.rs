use crate::day02::{Input, RPS, STATE};

pub fn solve(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| parse_rps_game(*line))
        .map(|game| evalute_game(game.0, game.1))
        .map(|outcome| (outcome.0 as u32) + (outcome.1 as u32))
        .sum()
}

fn parse_rps_game(line: (&str, &str)) -> (RPS, RPS) {
    let opponent_choice: RPS = match line.0 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let player_choise: RPS = match line.1 {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!(),
    };

    (opponent_choice, player_choise)
}

fn evalute_game(opponent_choice: RPS, player_choise: RPS) -> (STATE, RPS) {
    let win_lose_draw = match opponent_choice {
        RPS::Rock => match player_choise {
            RPS::Paper => STATE::Win,
            RPS::Scissors => STATE::Lost,
            _ => STATE::Draw,
        },
        RPS::Paper => match player_choise {
            RPS::Rock => STATE::Lost,
            RPS::Scissors => STATE::Win,
            _ => STATE::Draw,
        },
        RPS::Scissors => match player_choise {
            RPS::Rock => STATE::Win,
            RPS::Paper => STATE::Lost,
            _ => STATE::Draw,
        },
    };

    (win_lose_draw, player_choise)
}
