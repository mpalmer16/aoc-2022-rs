use aoc_common::fetch_with_transform;

fn main() {
    let moves = fetch_with_transform(9, transform);
    let visited: Vec<Pos> = vec![(0, 0)];

    let result = do_moves(&moves, 0, (0, 0), (0, 0), &visited);

    println!("answer 1: {}", result.len());

    let knots = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let result = do_moves_with_knots(&moves, 0, (0, 0), knots, &[(0, 0)]);

    println!("answer 2: {}", result.len());
}

fn transform(s: String) -> Vec<Dir> {
    s.split('\n').map(Dir::parse).collect::<Vec<Dir>>()
}

type Pos = (i32, i32);
type MoveReturn = (Pos, Pos, Dir, Vec<Pos>);
type MoveReturnWithKnots = (Pos, Vec<Pos>, Dir, Vec<Pos>);

fn do_moves(moves: &[Dir], idx: usize, head: Pos, tail: Pos, visited: &[Pos]) -> Vec<Pos> {
    if idx < moves.len() {
        let (new_head, new_tail, _, new_visited) =
            do_move(head, tail, moves[idx], visited.to_vec());
        do_moves(moves, idx + 1, new_head, new_tail, &new_visited)
    } else {
        visited.to_vec()
    }
}

fn do_moves_with_knots(
    moves: &[Dir],
    idx: usize,
    head: Pos,
    tail: Vec<Pos>,
    visited: &[Pos],
) -> Vec<Pos> {
    if idx < moves.len() {
        let (new_head, new_tail, _, new_visited) =
            do_move_with_knots(head, tail, moves[idx], visited.to_vec());
        do_moves_with_knots(moves, idx + 1, new_head, new_tail, &new_visited)
    } else {
        visited.to_vec()
    }
}

fn do_move(head: Pos, tail: Pos, dir: Dir, visited: Vec<Pos>) -> MoveReturn {
    if dir.no_more_steps() {
        (head, tail, dir, visited)
    } else {
        let (new_head, new_tail, new_dir, new_visited) = match dir {
            Dir::R(_) => apply((head.0, head.1 + 1), tail, dir, visited),
            Dir::U(_) => apply((head.0 + 1, head.1), tail, dir, visited),
            Dir::L(_) => apply((head.0, head.1 - 1), tail, dir, visited),
            Dir::D(_) => apply((head.0 - 1, head.1), tail, dir, visited),
        };
        do_move(new_head, new_tail, new_dir, new_visited)
    }
}

fn do_move_with_knots(
    head: Pos,
    tail: Vec<Pos>,
    dir: Dir,
    visited: Vec<Pos>,
) -> MoveReturnWithKnots {
    if dir.no_more_steps() {
        (head, tail, dir, visited)
    } else {
        let (new_head, new_tail, new_dir, new_visited) = match dir {
            Dir::R(_) => apply_with_knots((head.0, head.1 + 1), tail, dir, visited),
            Dir::U(_) => apply_with_knots((head.0 + 1, head.1), tail, dir, visited),
            Dir::L(_) => apply_with_knots((head.0, head.1 - 1), tail, dir, visited),
            Dir::D(_) => apply_with_knots((head.0 - 1, head.1), tail, dir, visited),
        };
        do_move_with_knots(new_head, new_tail, new_dir, new_visited)
    }
}

fn apply(head: Pos, tail: Pos, dir: Dir, visited: Vec<Pos>) -> MoveReturn {
    let new_tail = get_new_tail(head, tail);
    if !visited.contains(&new_tail) {
        (
            head,
            new_tail,
            dir.minus(1),
            vec![visited, vec![new_tail]].concat(),
        )
    } else {
        (head, new_tail, dir.minus(1), visited)
    }
}

fn apply_with_knots(head: Pos, tail: Vec<Pos>, dir: Dir, visited: Vec<Pos>) -> MoveReturnWithKnots {
    let new_tail = get_new_tail_with_knots(head, tail);
    if !visited.contains(new_tail.last().unwrap()) {
        (
            head,
            new_tail.clone(),
            dir.minus(1),
            vec![visited, vec![*new_tail.last().unwrap()]].concat(),
        )
    } else {
        (head, new_tail, dir.minus(1), visited)
    }
}

fn get_new_tail_with_knots(head: Pos, knots: Vec<Pos>) -> Vec<Pos> {
    let mut new_knots: Vec<Pos> = vec![];
    let mut next = get_new_tail(head, knots[0]);
    new_knots.push(next);
    for knot in knots[1..].iter() {
        next = get_new_tail(next, *knot);
        new_knots.push(next);
    }
    new_knots
}

fn get_new_tail((hx, hy): Pos, (tx, ty): Pos) -> Pos {
    match ((hx - tx), (hy - ty)) {
        (0, 0) | (1, 0) | (0, 1) | (-1, 0) | (0, -1) | (1, 1) | (-1, -1) | (-1, 1) | (1, -1) => {
            (tx, ty)
        }
        (diff, 0) => {
            let out_x = if diff > 0 { tx + 1 } else { tx - 1 };
            (out_x, ty)
        }
        (0, diff) => {
            let out_y = if diff > 0 { ty + 1 } else { ty - 1 };
            (tx, out_y)
        }
        (x_diff, y_diff) => {
            let out_x = if x_diff > 0 { tx + 1 } else { tx - 1 };
            let out_y = if y_diff > 0 { ty + 1 } else { ty - 1 };
            (out_x, out_y)
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    R(i32),
    U(i32),
    L(i32),
    D(i32),
}

impl Dir {
    fn parse(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "R" => Self::R(parts[1].parse::<i32>().unwrap()),
            "U" => Self::U(parts[1].parse::<i32>().unwrap()),
            "L" => Self::L(parts[1].parse::<i32>().unwrap()),
            "D" => Self::D(parts[1].parse::<i32>().unwrap()),
            _ => panic!("unknown!"),
        }
    }

    fn no_more_steps(&self) -> bool {
        match self {
            Dir::R(value) => *value == 0,
            Dir::U(value) => *value == 0,
            Dir::L(value) => *value == 0,
            Dir::D(value) => *value == 0,
        }
    }

    fn minus(&self, n: i32) -> Self {
        match self {
            Dir::R(m) => Dir::R(m - n),
            Dir::U(m) => Dir::U(m - n),
            Dir::L(m) => Dir::L(m - n),
            Dir::D(m) => Dir::D(m - n),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{do_move, do_moves, do_moves_with_knots, transform, Dir};

    const TEST_FILE: &str = "inputs/test_input.txt";
    const LARGER_TEST_FILE: &str = "inputs/test_input_2.txt";

    #[test]
    fn can_read_input() {
        let input = get_test_input(TEST_FILE, transform);

        assert!(input.len() == 8);
    }

    #[test]
    fn can_follow_direction_r() {
        let dir = Dir::R(4);
        let (_, _, _, result) = do_move((0, 0), (0, 0), dir, vec![(0, 0)]);

        assert!(result.len() == 4);
    }

    #[test]
    fn can_follow_direction_l() {
        let dir = Dir::L(4);
        let (_, _, _, result) = do_move((0, 0), (0, 0), dir, vec![(0, 0)]);

        assert!(result.len() == 4);
    }

    #[test]
    fn can_follow_on_diagonal() {
        let moves = vec![Dir::L(2), Dir::U(2)];
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 3);
    }

    #[test]
    fn can_follow_up_right() {
        let moves = vec![Dir::U(1), Dir::R(1), Dir::U(1)];
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 2);
        assert!(result == vec![(0, 0), (1, 1)])
    }

    #[test]
    fn can_follow_down_right() {
        let moves = vec![Dir::D(1), Dir::R(1), Dir::D(1)];
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 2);
        assert!(result == vec![(0, 0), (-1, 1)])
    }

    #[test]
    fn can_follow_up_left() {
        let moves = vec![Dir::U(1), Dir::L(1), Dir::U(1)];
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 2);
        assert!(result == vec![(0, 0), (1, -1)])
    }

    #[test]
    fn can_follow_down_left() {
        let moves = vec![Dir::D(1), Dir::L(1), Dir::D(1)];
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 2);
        assert!(result == vec![(0, 0), (-1, -1)])
    }

    #[test]
    fn can_do_moves() {
        let moves = get_test_input(TEST_FILE, transform);
        let result = do_moves(&moves, 0, (0, 0), (0, 0), &[(0, 0)]);

        assert!(result.len() == 13);
    }

    #[test]
    fn can_move_ten_knots() {
        let moves = get_test_input(LARGER_TEST_FILE, transform);
        let knots = vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        let result = do_moves_with_knots(&moves, 0, (0, 0), knots, &[(0, 0)]);

        assert!(result.len() == 36);
    }
}
