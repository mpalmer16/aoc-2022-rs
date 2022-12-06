use std::fmt::Display;

use aoc_common::fetch_with_transform;

fn main() {
    let answer_1 = get_answer_1();

    println!("answer 1: {:?}", answer_1);

    let answer_2 = get_answer_2();

    println!("answer 2: {:?}", answer_2);
}

fn get_answer_1() -> String {
    let (mut stacks, moves) = fetch_with_transform(5, stack_transform);

    for mv in moves {
        stacks = do_move(stacks, mv);
    }

    let tops: Vec<&Crate> = stacks.iter().map(|stack| stack.1.last().unwrap()).collect();

    tops.iter()
        .map(|&t| t.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn get_answer_2() -> String {
    let (mut stacks, moves) = fetch_with_transform(5, stack_transform);

    for mv in moves {
        stacks = do_move_part_2(stacks, mv);
    }

    let tops: Vec<&Crate> = stacks.iter().map(|stack| stack.1.last().unwrap()).collect();

    tops.iter()
        .map(|&t| t.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn stack_transform(s: String) -> (Vec<(usize, Stack)>, Moves) {
    let parts = s
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    assert!(parts.len() == 2);

    let stack_string = &parts[0];
    let stacks = get_stacks(stack_string);

    let move_string = &parts[1];
    let moves = get_moves(move_string);

    (stacks, moves)
}

fn get_stacks(stack_string: &str) -> Vec<(usize, Stack)> {
    let stack_strings_with_numbers = stack_string.split('\n').collect::<Vec<&str>>();

    let stack_strings = stack_strings_with_numbers
        .iter()
        .take(stack_strings_with_numbers.len() - 1)
        .copied()
        .collect::<Vec<&str>>();

    let stacks = stack_strings
        .iter()
        .map(|&s| s.split("  ").map(|s| s.trim()).collect::<Vec<&str>>())
        .map(|stack| stack.chunks(2).map(|c| c.join(" ")).collect::<Vec<_>>())
        .map(|stack| {
            stack
                .iter()
                .flat_map(|s| {
                    if s == " " {
                        vec![Crate::Nothing]
                    } else {
                        s.split(' ').map(Crate::from).collect::<Vec<Crate>>()
                    }
                })
                .collect::<Vec<Crate>>()
        })
        .collect::<Vec<Vec<Crate>>>();

    let pivoted = pivot(stacks);
    remove_nothings(pivoted)
}

fn pivot(stacks: Vec<Stack>) -> Vec<Vec<Crate>> {
    let mut pivoted: Vec<Stack> = vec![];
    for i in 0..=stacks.len() {
        let mut new_stack: Stack = vec![];
        for stack in &stacks {
            new_stack.push(stack[i]);
        }
        new_stack.reverse();
        pivoted.push(new_stack);
    }
    pivoted
}

fn remove_nothings(stacks: Vec<Stack>) -> Vec<(usize, Stack)> {
    stacks
        .iter()
        .map(|s| {
            s.iter()
                .filter(|&&c| c != Crate::Nothing)
                .copied()
                .collect::<Stack>()
        })
        .enumerate()
        .collect::<Vec<(usize, Stack)>>()
}

fn get_moves(move_string: &str) -> Moves {
    move_string
        .split('\n')
        .map(|s| s.split(" from ").collect::<Vec<&str>>())
        .map(|move_parts| {
            assert!(move_parts.len() == 2);
            let how_many = move_parts[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .expect("can not parse stack as usize");
            let from_to_str = move_parts[1].split(" to ").collect::<Vec<&str>>();
            let from = from_to_str[0]
                .parse::<usize>()
                .expect("could not parse from as usize")
                - 1;
            let to = from_to_str[1]
                .parse::<usize>()
                .expect("could not parse to as usize")
                - 1;
            Move::from(how_many, from, to)
        })
        .collect::<Vec<Move>>()
}

fn do_move(stacks: Vec<(usize, Stack)>, mv: Move) -> Vec<(usize, Stack)> {
    let mut from_stack = stacks[mv.from].clone();
    let mut to_stack = stacks[mv.to].clone();
    let mut others: Vec<(usize, Stack)> = vec![];

    let mut count = mv.how_many;

    while count > 0 {
        let c = from_stack.1.pop().unwrap();
        if c != Crate::Nothing {
            to_stack.1.push(c);
            count -= 1;
        }
    }

    for (i, stack) in stacks.iter().enumerate() {
        if i != mv.from && i != mv.to {
            others.push(stack.clone());
        }
    }
    let mut result = vec![vec![from_stack], vec![to_stack], others].concat();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

fn do_move_part_2(stacks: Vec<(usize, Stack)>, mv: Move) -> Vec<(usize, Stack)> {
    let mut from_stack = stacks[mv.from].clone();
    let mut to_stack = stacks[mv.to].clone();
    let mut others: Vec<(usize, Stack)> = vec![];

    let count = mv.how_many;

    let to_move = &from_stack.1[(from_stack.1.len() - count)..];
    to_stack.1 = [to_stack.1, to_move.to_vec()].concat();
    from_stack.1 = from_stack.1[..(from_stack.1.len() - count)].to_vec();

    for (i, stack) in stacks.iter().enumerate() {
        if i != mv.from && i != mv.to {
            others.push(stack.clone());
        }
    }
    let mut result = vec![vec![from_stack], vec![to_stack], others].concat();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use aoc_common::get_test_input;

    use crate::{do_move, do_move_part_2, get_stacks, stack_transform, Crate};

    const TEST_FILE: &str = "inputs/test_input.txt";

    #[test]
    fn can_read_input() {
        let (stacks, moves) = get_test_input(TEST_FILE, stack_transform);

        assert!(stacks.len() == 9 && moves.len() == 2);
    }

    #[test]
    fn can_parse_stacks_and_pivot() {
        let s = read_to_string(TEST_FILE).unwrap();
        let parts = s
            .split("\n\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert!(parts.len() == 2);

        let stack_string = &parts[0];
        let stacks = get_stacks(stack_string);

        assert!(stacks[0].1[4] == Crate::R);
        assert!(stacks[4].1.last().unwrap() == &Crate::M);
        assert!(stacks[8].1.last().unwrap() == &Crate::L);
    }

    #[test]
    fn can_move_stacks() {
        let (mut stacks, moves) = get_test_input(TEST_FILE, stack_transform);

        for mv in moves {
            stacks = do_move(stacks, mv);
        }

        assert!(stacks[2].1.is_empty());
        assert!(stacks[7].1.len() == 3);
    }

    #[test]
    fn can_move_stacks_part_2() {
        let (mut stacks, moves) = get_test_input(TEST_FILE, stack_transform);

        for mv in moves {
            stacks = do_move_part_2(stacks, mv);
        }

        assert!(stacks[2].1.is_empty());
        assert!(stacks[7].1.len() == 3);
    }
}

type Stack = Vec<Crate>;
type Moves = Vec<Move>;

#[derive(Debug)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from(how_many: usize, from: usize, to: usize) -> Self {
        Self { how_many, from, to }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Crate {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Nothing,
}

impl Crate {
    fn from(s: &str) -> Self {
        match s {
            "" => Self::Nothing,
            "[A]" => Self::A,
            "[B]" => Self::B,
            "[C]" => Self::C,
            "[D]" => Self::D,
            "[E]" => Self::E,
            "[F]" => Self::F,
            "[G]" => Self::G,
            "[H]" => Self::H,
            "[I]" => Self::I,
            "[J]" => Self::J,
            "[K]" => Self::K,
            "[L]" => Self::L,
            "[M]" => Self::M,
            "[N]" => Self::N,
            "[O]" => Self::O,
            "[P]" => Self::P,
            "[Q]" => Self::Q,
            "[R]" => Self::R,
            "[S]" => Self::S,
            "[T]" => Self::T,
            "[U]" => Self::U,
            "[V]" => Self::V,
            "[W]" => Self::W,
            "[X]" => Self::X,
            "[Y]" => Self::Y,
            "[Z]" => Self::Z,
            _ => panic!("unknown crate name: {}", s),
        }
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nothing => write!(f, "Nothing"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::H => write!(f, "H"),
            Self::I => write!(f, "I"),
            Self::J => write!(f, "J"),
            Self::K => write!(f, "K"),
            Self::L => write!(f, "L"),
            Self::M => write!(f, "M"),
            Self::N => write!(f, "N"),
            Self::O => write!(f, "O"),
            Self::P => write!(f, "P"),
            Self::Q => write!(f, "Q"),
            Self::R => write!(f, "R"),
            Self::S => write!(f, "S"),
            Self::T => write!(f, "T"),
            Self::U => write!(f, "U"),
            Self::V => write!(f, "V"),
            Self::W => write!(f, "W"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::Z => write!(f, "Z"),
        }
    }
}
