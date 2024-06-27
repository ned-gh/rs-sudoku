#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CellCandidate {
    row: u32,
    col: u32,
    val: u32,
}

impl CellCandidate {
    pub fn new(row: u32, col: u32, val: u32) -> CellCandidate {
        CellCandidate { row, col, val }
    }
}
