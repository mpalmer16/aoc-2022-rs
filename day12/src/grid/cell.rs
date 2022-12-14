
#[derive(Clone, Copy)]
pub enum Cell {
    Start,
    End,
    Square(u8),
}

impl Cell {
    pub fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(e) => e
        }
    }
}