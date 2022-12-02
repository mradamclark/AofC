use crate::day02::{Input, RPS, STATE};

pub fn solve(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| parse_rps_game(*line))
        .map(|game| evalute_game(game.0, game.1))
        .map(|outcome| (outcome.0 as u32) + (outcome.1 as u32))
        .sum()
}

fn parse_rps_game(game: (&str, &str)) -> (RPS, STATE) {
    let opponent_choice: RPS = match game.0 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let player_choice: STATE = match game.1 {
        "X" => STATE::Lost,
        "Y" => STATE::Draw,
        "Z" => STATE::Win,
        _ => panic!(),
    };

    (opponent_choice, player_choice)
}

fn evalute_game(opponent_choice: RPS, player_needs_to: STATE) -> (STATE, RPS) {
    let player_choice = match opponent_choice {
        RPS::Rock => match player_needs_to {
            STATE::Win => RPS::Paper,
            STATE::Lost => RPS::Scissors,
            _ => RPS::Rock,
        },
        RPS::Paper => match player_needs_to {
            STATE::Win => RPS::Scissors,
            STATE::Lost => RPS::Rock,
            _ => RPS::Paper,
        },
        RPS::Scissors => match player_needs_to {
            STATE::Win => RPS::Rock,
            STATE::Lost => RPS::Paper,
            _ => RPS::Scissors,
        },
    };

    (player_needs_to, player_choice)
}
