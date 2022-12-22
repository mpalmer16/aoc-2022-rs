use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt,
};

/// Got stuck on this one, but found
/// [this](https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2022/src/day_12.rs)
/// very helpful code (used here)
fn main() {
    let input = include_str!("../inputs/test_input.txt");
    let data = Grid::new(input.to_string()).unwrap();

    println!("answer 1: {}", data.start_to_end());
    println!("answer 2: {}", data.end_to_start());
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord {
    fn neighbors(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            })
        }

        // down
        if self.y < rows - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // right
        if self.x < cols - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        result.into_iter()
    }
}

struct Grid {
    map: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
    rows: usize,
    cols: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} Grid:", self.rows, self.cols)?;
        for y in 0..self.rows {
            for x in 0..self.cols {
                let height = self.map[y][x];
                write!(f, "{height}\t")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: String) -> Option<Self> {
        let rows = input.lines().count();
        let cols = input.lines().next()?.len();
        let mut map = vec![vec![0; cols]; rows];
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let letter = match c {
                    'S' => {
                        start.x = col;
                        start.y = row;
                        'a'
                    }
                    'E' => {
                        end.x = col;
                        end.y = row;
                        'z'
                    }
                    'a'..='z' => c,
                    _ => return None,
                };
                let val = letter as u8 - b'a';
                map[row][col] = val;
            }
        }
        Some(Self {
            map,
            start,
            end,
            rows,
            cols,
        })
    }

    fn start_to_end(&self) -> String {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: self.start,
        });
        visited.insert(self.start);

        while let Some(Node { coord, cost }) = pq.pop() {
            if coord == self.end {
                return cost.to_string();
            }
            let curr_height = self.map[coord.y][coord.x];
            let neighbors = coord.neighbors(self.rows, self.cols);
            let candidates = neighbors.into_iter().filter(|coord| {
                let height = self.map[coord.y][coord.x];
                height <= curr_height || height == curr_height + 1
            });

            for candidate in candidates {
                if visited.insert(candidate) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: candidate,
                    })
                }
            }
        }
        "no path found".to_string()
    }

    fn end_to_start(&self) -> String {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: self.end,
        });
        visited.insert(self.end);

        while let Some(Node { coord, cost }) = pq.pop() {
            let curr_height = self.map[coord.y][coord.x];
            if curr_height == 0 {
                return cost.to_string();
            }
            let neighbors = coord.neighbors(self.rows, self.cols);
            let candidates = neighbors.into_iter().filter(|coord| {
                let height = self.map[coord.y][coord.x];
                height >= curr_height || height == curr_height - 1
            });

            for candidate in candidates {
                if visited.insert(candidate) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: candidate,
                    })
                }
            }
        }

        "path not found".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn can_answer_part_1() {
        let data = Grid::new(include_str!("../inputs/test_input.txt").to_string()).unwrap();

        dbg!(&data);
        assert!(data.start_to_end() == "31");
    }

    #[test]
    fn can_answer_part_2() {
        let data = Grid::new(include_str!("../inputs/test_input.txt").to_string()).unwrap();

        dbg!(&data);
        assert!(data.end_to_start() == "29");
    }
}
