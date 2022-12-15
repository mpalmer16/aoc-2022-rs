mod cell;
mod grid_coord;

use std::{fmt, collections::{HashMap, HashSet}};

use self::{cell::Cell, grid_coord::GridCoord};

struct CellRecord {
    _prev: Option<GridCoord>,
}


pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    _visited: HashMap<GridCoord, CellRecord>,
    _current: HashSet<GridCoord>,
    _num_steps: usize
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let first_line = input.lines().next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();
        let mut data = vec![];
        for c in input.chars() {
            let cell = match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Square(c as u8 - b'a'),
                '\r' | '\n' => continue,
                _ => panic!("invalid character: {}", c),
            };
            data.push(cell);
        }
        Self {
            width,
            height,
            data,
            _current: Default::default(),
            _visited: Default::default(),
            _num_steps: 0
        }
    }

    pub fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub fn _cell_mut(&mut self, coord: GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    
    // fn _walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
    //     let curr_elev = self.cell(coord).unwrap().elevation();
    //     let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    //     deltas.into_iter().filter_map(move |(dx, dy)| {
    //         Some(GridCoord {
    //             x: coord.x.checked_add_signed(dx)?,
    //             y: coord.y.checked_add_signed(dy)?
    //         })
    //     })
    //     .filter(|&coord| self.in_bounds(coord))
    //     .filter(|&coord| {
    //         let other_elev = self.cell(coord).unwrap().elevation();
    //         other_elev <= curr_elev + 1
    //     })
    // }

    fn _step (&mut self) {
        if self._current.is_empty() {
            let mut _start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x,y).into();
                    if let Cell::Start = self.cell(coord).unwrap() {
                        _start_coord = Some(coord);
                        break;
                    }
                }
            }
            let _start_coord = _start_coord.unwrap();
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}