
mod grid;
mod scratch;


use crate::grid::Grid;

/// Another difficult one for me - my original attempt (with passing tests!) is in the scratch module
/// this one is from the once again wonderful
/// [fasterthanlime](https://fasterthanli.me/series/advent-of-code-2022/part-12)
fn main() {
    let grid = Grid::parse(include_str!("../inputs/test_input.txt"));
    println!("{grid:?}");
}

