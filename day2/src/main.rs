use aoc_common::fetch_with_transform;

use crate::RPS::{Paper, Rock, Scissors};
use crate::WLD::{Draw, Lose, Win};

fn main() {
    let transform = |s: String| {
        s.split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|cs| {
                assert!(cs.len() == 2);
                (RPS::read(cs[0]), RPS::read(cs[1]))
            })
            .collect::<Vec<(RPS, RPS)>>()
    };

    let score = fetch_with_transform(2, transform)
        .iter()
        .map(get_score)
        .sum::<i32>();

    println!("answer 1: {}", score);

    let transform = |s: String| {
        s.split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|cs| {
                assert!(cs.len() == 2);
                (RPS::read(cs[0]), WLD::read(cs[1]))
            })
            .collect::<Vec<(RPS, WLD)>>()
    };

    let score = fetch_with_transform(2, transform)
        .iter()
        .map(|round| (round.0, round.1.convert(&round.0)))
        .map(|round| get_score(&round))
        .sum::<i32>();

    println!("answer 2: {}", score);
}

fn get_score_for_shape(round: &(RPS, RPS)) -> i32 {
    match round.1 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn get_score_for_outcome(round: &(RPS, RPS)) -> i32 {
    match round {
        (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 0,
    }
}

fn get_score(round: &(RPS, RPS)) -> i32 {
    get_score_for_shape(round) + get_score_for_outcome(round)
}

#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn read(c: &str) -> Self {
        match c {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(PartialEq)]
enum WLD {
    Win,
    Lose,
    Draw,
}

impl WLD {
    fn read(c: &str) -> Self {
        match c {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("invalid input"),
        }
    }

    fn convert(&self, other: &RPS) -> RPS {
        match self {
            Self::Lose => match other {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Self::Draw => *other,
            Self::Win => match other {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{
        get_score,
        RPS::{self, Paper, Rock, Scissors},
        WLD::{self, Draw, Lose, Win},
    };

    const TEST_INPUT_FILE: &str = "inputs/test_input.txt";

    fn test_transform_rps(s: String) -> Vec<(RPS, RPS)> {
        s.split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|cs| {
                assert!(cs.len() == 2);
                (RPS::read(cs[0]), RPS::read(cs[1]))
            })
            .collect::<Vec<(RPS, RPS)>>()
    }

    fn test_transform_wld(s: String) -> Vec<(RPS, WLD)> {
        s.split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|v| {
                assert!(v.len() == 2);
                (RPS::read(v[0]), WLD::read(v[1]))
            })
            .collect::<Vec<(RPS, WLD)>>()
    }

    #[test]
    fn can_read_rps_input() {
        let input: Vec<(RPS, RPS)> = get_test_input(TEST_INPUT_FILE, test_transform_rps);
        assert!(input == vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)])
    }

    #[test]
    fn can_read_wld_input() {
        let input: Vec<(RPS, WLD)> = get_test_input(TEST_INPUT_FILE, test_transform_wld);
        assert!(input == vec![(Rock, Draw), (Paper, Lose), (Scissors, Win)]);
    }

    #[test]
    fn can_get_scores() {
        let input: Vec<(RPS, RPS)> = get_test_input(TEST_INPUT_FILE, test_transform_rps);
        let scores = input.iter().map(get_score).collect::<Vec<i32>>();

        assert!(scores == vec![8, 1, 6]);
        assert!(scores.iter().sum::<i32>() == 15);
    }

    #[test]
    fn can_decrypt_rounds_with_wld_to_rps() {
        let input: Vec<(RPS, WLD)> = get_test_input(TEST_INPUT_FILE, test_transform_wld);

        let scores = input
            .iter()
            .map(|round| (round.0, round.1.convert(&round.0)))
            .map(|round| get_score(&round))
            .collect::<Vec<i32>>();

        assert!(scores == vec![4, 1, 7]);
        assert!(scores.iter().sum::<i32>() == 12);
    }
}
