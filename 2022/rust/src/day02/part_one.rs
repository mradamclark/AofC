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

fn evalute_game(opponent_choice: RPS, player_choice: RPS) -> (STATE, RPS) {
    let player_outcome = match (opponent_choice, player_choice) {
        (RPS::Rock, RPS::Paper) => STATE::Win,
        (RPS::Rock, RPS::Scissors) => STATE::Lost,
        (RPS::Paper, RPS::Rock) => STATE::Lost,
        (RPS::Paper, RPS::Scissors) => STATE::Win,
        (RPS::Scissors, RPS::Rock) => STATE::Win,
        (RPS::Scissors, RPS::Paper) => STATE::Lost,
        _ => STATE::Draw,
    };

    (player_outcome, player_choice)
}
