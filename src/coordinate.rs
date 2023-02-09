#[derive(Debug, Copy, Clone, std::hash::Hash)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl Coordinate {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}{:0>2}", self.row, self.col)
    }
}
