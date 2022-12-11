use std::{fmt::Display, str::FromStr};

use aoc_common::fetch_with_transform;

fn main() {
    let input = fetch_with_transform(10, |s| {
        s.split('\n').map(|s| s.parse::<Inst>().unwrap()).collect()
    });
    let (interesting_signals, crt_screen) = run(input);

    let answer_1 = interesting_signals.iter().sum::<i32>();

    println!("answer 1: {answer_1}");

    println!("answer 2:");
    for line in crt_screen {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

fn run(instructions: Vec<Inst>) -> (Vec<i32>, Vec<Vec<char>>) {
    let mut value = 1;
    let mut cycle = 1;
    let mut interesting_signals: Vec<i32> = vec![];
    let mut crt_screen: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    let mut crt_idx: (usize, usize) = (0, 0);

    for instruction in instructions {
        match instruction {
            Inst::AddX(x) => {
                check_cycle(
                    &mut cycle,
                    value,
                    &mut interesting_signals,
                    &mut crt_screen,
                    &mut crt_idx,
                );
                check_cycle(
                    &mut cycle,
                    value,
                    &mut interesting_signals,
                    &mut crt_screen,
                    &mut crt_idx,
                );
                value += x;
            }
            Inst::Noop => {
                check_cycle(
                    &mut cycle,
                    value,
                    &mut interesting_signals,
                    &mut crt_screen,
                    &mut crt_idx,
                );
            }
        }
    }
    (interesting_signals, crt_screen)
}

fn check_cycle(
    cycle: &mut i32,
    value: i32,
    interesting_signals: &mut Vec<i32>,
    crt_screen: &mut [Vec<char>],
    (x, y): &mut (usize, usize),
) {
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];

    if interesting_cycles.contains(cycle) {
        interesting_signals.push(*cycle * value);
    }

    if sprite_is_on_crt(value, *y) {
        crt_screen[*x][*y] = '#';
    }

    if *y == 39 {
        *x += 1;
        *y = 0;
    } else {
        *y += 1;
    }
    *cycle += 1;
}

fn sprite_is_on_crt(mid_sprite: i32, crt_pos: usize) -> bool {
    ((mid_sprite - 1) as usize) == crt_pos
        || (mid_sprite as usize) == crt_pos
        || ((mid_sprite + 1) as usize) == crt_pos
}

#[derive(PartialEq)]
enum Inst {
    AddX(i32),
    Noop,
}

impl FromStr for Inst {
    type Err = InstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "addx" => Ok(Self::AddX(parts[1].parse().unwrap())),
            "noop" => Ok(Self::Noop),
            _ => Err(InstError::with(parts[0])),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InstError {
    value: String,
}

impl InstError {
    fn with(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Display for InstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid instruction: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{run, Inst, InstError};

    const TEST_FILE: &str = "inputs/test_input.txt";

    fn transform(s: String) -> Vec<Inst> {
        s.split('\n').map(|s| s.parse::<Inst>().unwrap()).collect()
    }

    #[test]
    fn fail_on_bad_instructions() {
        let result = "foo bar".parse::<Inst>();

        assert!(
            result
                == Err(InstError {
                    value: "foo".to_string()
                })
        );
    }

    #[test]
    fn can_read_instruction() {
        let result = "addx 12".parse::<Inst>();

        assert!(result == Ok(Inst::AddX(12)));
    }

    #[test]
    fn can_read_input() {
        let input = get_test_input(TEST_FILE, transform);

        assert!(input.len() == 146);
    }

    #[test]
    fn can_cycle_instructions() {
        let input = get_test_input(TEST_FILE, transform);
        let (interesting_signals, _) = run(input);

        assert!(interesting_signals.iter().sum::<i32>() == 13140);
    }

    #[test]
    fn can_draw_crt_screen() {
        let input = get_test_input(TEST_FILE, transform);
        let (_, crt_screen) = run(input);

        assert!(crt_screen[0][0] == '#');
        assert!(crt_screen[0][1] == '#');
        assert!(crt_screen[0][2] == ' ');
    }
}
