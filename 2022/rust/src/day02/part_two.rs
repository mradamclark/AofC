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
        "X" => STATE::Lose,
        "Y" => STATE::Draw,
        "Z" => STATE::Win,
        _ => panic!(),
    };

    (opponent_choice, player_choice)
}

fn evalute_game(opponent_choice: RPS, player_needs_to: STATE) -> (STATE, RPS) {
   let player_choice = match (opponent_choice, player_needs_to) {
            (RPS::Rock, STATE::Win) => RPS::Paper,
            (RPS::Rock, STATE::Lose) => RPS::Scissors,
            (RPS::Paper, STATE::Win) => RPS::Scissors,
            (RPS::Paper, STATE::Lose) => RPS::Rock,
            (RPS::Scissors, STATE::Win) => RPS::Rock,
            (RPS::Scissors, STATE::Lose) => RPS::Paper,
            (_, STATE::Draw) => opponent_choice
        };

    (player_needs_to, player_choice)
}
