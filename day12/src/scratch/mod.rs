use std::collections::HashMap;

pub fn _find_position_of(map: &[Vec<i32>], n: i32) -> (usize, usize) {
    for (x, row) in map.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == n {
                return (x, y);
            }
        }
    }
    panic!("position not found!")
}

pub fn _elevations(s: String) -> Vec<Vec<i32>> {
    let elevation_lookup: HashMap<char, i32> =
        ('a'..='z').zip(1..=26).collect::<HashMap<char, i32>>();
    s.split('\n')
        .map(|line| {
            line.split("")
                .filter(|&c| !c.is_empty())
                .map(|c| _parse_elevation(c, &elevation_lookup))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn _find_paths(
    (x, y): (usize, usize),
    map: &Vec<Vec<i32>>,
    points: &[(usize, usize)],
) -> Vec<Vec<(usize, usize)>> {
    let ps = vec![points.to_owned(), vec![(x, y)]].concat();
    let next_moves = _get_next_moves((x, y), map, &ps);
    if next_moves.is_empty() {
        vec![ps.to_vec()]
    } else {
        next_moves
            .iter()
            .flat_map(|&m| _find_paths(m, map, &ps))
            .collect::<Vec<_>>()
    }
}

fn _parse_elevation(s: &str, elevation_lookup: &HashMap<char, i32>) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let c = chars.first().expect("single char not found");
    match c {
        'S' => 0,
        'E' => 27,
        _ => *elevation_lookup.get(c).expect("unknown elevation value"),
    }
}

fn _get_next_moves(
    (x, y): (usize, usize),
    map: &[Vec<i32>],
    visited: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    if map[x][y] == 27 {
        vec![]
    } else if _is_on_edge((x, y), map) {
        match (x, y) {
            (0, 0) => _filter_available(vec![(x + 1, y), (x, y + 1)], (x, y), map, visited),
            (x, 0) => {
                if x == map.len() - 1 {
                    _filter_available(vec![(x - 1, y), (x, y + 1)], (x, y), map, visited)
                } else {
                    _filter_available(
                        vec![(x + 1, y), (x - 1, y), (x, y + 1)],
                        (x, y),
                        map,
                        visited,
                    )
                }
            }
            (0, y) => {
                if y == map[x].len() - 1 {
                    _filter_available(vec![(x + 1, y - 1), (x, y - 1)], (x, y), map, visited)
                } else {
                    _filter_available(
                        vec![(x, y + 1), (x, y - 1), (x + 1, y)],
                        (x, y),
                        map,
                        visited,
                    )
                }
            }
            (x, y) => {
                if x == map.len() - 1 && y == map[x].len() - 1 {
                    _filter_available(vec![(x - 1, y), (x, y - 1)], (x, y), map, visited)
                } else if x == map.len() - 1 {
                    _filter_available(
                        vec![(x - 1, y), (x, y - 1), (x, y + 1)],
                        (x, y),
                        map,
                        visited,
                    )
                } else {
                    //if y == map[x].len() {
                    _filter_available(
                        vec![(x - 1, y), (x + 1, y), (x, y - 1)],
                        (x, y),
                        map,
                        visited,
                    )
                }
            }
        }
    } else {
        _filter_available(
            vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)],
            (x, y),
            map,
            visited,
        )
    }
}

fn _is_on_edge((x, y): (usize, usize), map: &[Vec<i32>]) -> bool {
    x == 0 || y == 0 || x == map.len() - 1 || y == map[x].len() - 1
}

fn _filter_available(
    compares: Vec<(usize, usize)>,
    point: (usize, usize),
    map: &[Vec<i32>],
    visited: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    compares
        .iter()
        .filter(|&&c| _available(point, (c.0, c.1), map, visited))
        .copied()
        .collect::<Vec<(usize, usize)>>()
}

fn _available(
    point: (usize, usize),
    compare: (usize, usize),
    map: &[Vec<i32>],
    visited: &[(usize, usize)],
) -> bool {
    let p = map[point.0][point.1];
    let c = map[compare.0][compare.1];
    let already_been_there = visited.contains(&compare);

    !already_been_there && (c <= p || c - p == 1)
}

#[cfg(test)]
mod tests {
    extern crate aoc_common;
    use self::aoc_common::get_test_input;

    use crate::scratch::{_elevations, _find_paths, _find_position_of};

    const TEST_FILE: &str = "inputs/test_input.txt";
    const _TEST_FILE_SMALL: &str = "inputs/test_input_small.txt";

    #[test]
    fn _can_read_input() {
        let elevation_map = get_test_input(TEST_FILE, _elevations);

        assert!(elevation_map.len() == 5);
        assert!(elevation_map[0].len() == 8);

        for line in elevation_map {
            for value in line {
                if value > 9 {
                    print!(" {value} ");
                } else {
                    print!(" {value}  ");
                }
            }
            println!();
        }
    }

    #[test]
    fn _can_find_paths() {
        let elevation_map = get_test_input(TEST_FILE, _elevations);
        let start = _find_position_of(&elevation_map, 0);
        let end = _find_position_of(&elevation_map, 27);

        let paths = _find_paths(start, &elevation_map, &[]);
        let mut paths = paths
            .iter()
            .filter(|p| *p.last().unwrap() == end)
            .collect::<Vec<_>>();
        paths.sort_by_key(|p| p.len());

        assert!(paths.first().unwrap().len() - 1 == 31);
    }
}
