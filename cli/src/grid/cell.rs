use crate::util::BitSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellCandidate {
    row: u32,
    col: u32,
    val: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    row: u32,
    col: u32,
    candidates: BitSet,
}

impl CellCandidate {
    pub fn from(row: u32, col: u32, val: u32) -> CellCandidate {
        CellCandidate { row, col, val }
    }

    pub fn from_cell(cell: &Cell, val: u32) -> CellCandidate {
        CellCandidate {
            row: cell.get_row(),
            col: cell.get_col(),
            val,
        }
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_col(&self) -> u32 {
        self.col
    }

    pub fn get_val(&self) -> u32 {
        self.val
    }

    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.row, self.col, self.val)
    }
}

impl Cell {
    pub fn from(row: u32, col: u32, candidates: &BitSet) -> Cell {
        Cell {
            row,
            col,
            candidates: candidates.clone(),
        }
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_col(&self) -> u32 {
        self.col
    }

    pub fn get_candidates(&self) -> &BitSet {
        &self.candidates
    }
}
