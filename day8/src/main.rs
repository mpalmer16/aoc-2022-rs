use aoc_common::fetch_with_transform;

fn main() {
    let input = fetch_with_transform(8, read_map);

    let visibility_map = visibility(input);

    let answer_1 = visibility_map
        .iter()
        .map(|v| v.iter().filter(|&&p| p.0).count())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    println!("answer 1: {answer_1}");

    let highest_scenic_score = visibility_map
        .iter()
        .map(|v| v.iter().filter(|&&p| p.0))
        .map(|v| v.max_by(|a, b| a.1.cmp(&b.1)).unwrap())
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    println!("answer 2: {highest_scenic_score}");
}

fn read_map(s: String) -> Vec<Vec<i32>> {
    s.split('\n')
        .map(|s| {
            s.chars()
                .map(|s| s.to_string().parse::<i32>().expect("could not parse int"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn visibility(map: Vec<Vec<i32>>) -> Vec<Vec<(bool, i32)>> {
    let mut visible_map: Vec<Vec<(bool, i32)>> = vec![];

    for (x, row) in map.iter().enumerate() {
        let mut visible: Vec<(bool, i32)> = vec![];

        for (y, value) in row.iter().enumerate() {
            visible.push(is_visible((x, y, *value), &map));
        }
        visible_map.push(visible);
    }
    visible_map
}

fn is_visible(point: (usize, usize, i32), map: &Vec<Vec<i32>>) -> (bool, i32) {
    if on_edge(point.0, point.1, map) {
        (true, 0)
    } else {
        let look_around = vec![
            look(Direction::Up, point, map, 1),
            look(Direction::Down, point, map, 1),
            look(Direction::Left, point, map, 1),
            look(Direction::Right, point, map, 1),
        ];

        let visible = look_around.iter().any(|v| v.0);
        let scenic_score = look_around.iter().map(|v| v.1).product::<i32>();

        (visible, scenic_score)
    }
}

fn on_edge(x: usize, y: usize, map: &Vec<Vec<i32>>) -> bool {
    x == 0 || y == 0 || x == map.len() - 1 || y == map[0].len() - 1
}

fn next(
    direction: Direction,
    x: usize,
    y: usize,
    value: i32,
    map: &Vec<Vec<i32>>,
    count: i32,
) -> (bool, i32) {
    if map[x][y] < value {
        if on_edge(x, y, map) {
            (true, count)
        } else {
            look(direction, (x, y, value), map, count + 1)
        }
    } else {
        (false, count)
    }
}

fn look(
    direction: Direction,
    (x, y, value): (usize, usize, i32),
    map: &Vec<Vec<i32>>,
    count: i32,
) -> (bool, i32) {
    match direction {
        Direction::Up => next(Direction::Up, x + 1, y, value, map, count),
        Direction::Down => next(Direction::Down, x - 1, y, value, map, count),
        Direction::Left => next(Direction::Left, x, y - 1, value, map, count),
        Direction::Right => next(Direction::Right, x, y + 1, value, map, count),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{read_map, visibility};

    const TEST_FILE: &str = "inputs/test_input.txt";

    #[test]
    fn can_read_input() {
        let input = get_test_input(TEST_FILE, read_map);

        assert!(input.len() == 5);
    }

    #[test]
    fn can_get_visibility() {
        let input = get_test_input(TEST_FILE, read_map);

        let visibility_map = visibility(input);

        let visible = visibility_map
            .iter()
            .map(|v| v.iter().filter(|&&p| p.0).count())
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>();

        assert!(visible == 21);
    }

    #[test]
    fn can_get_highest_scenic_score() {
        let input = get_test_input(TEST_FILE, read_map);

        let visibility_map = visibility(input);

        let highest_scenic_score = visibility_map
            .iter()
            .map(|v| v.iter().filter(|&&p| p.0))
            .map(|v| v.max_by(|a, b| a.1.cmp(&b.1)).unwrap())
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        assert!(highest_scenic_score.1 == 8);
    }
}
