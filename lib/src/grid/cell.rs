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

    pub fn can_see(&self, other: &CellCandidate, check_val: bool) -> bool {
        if check_val && self.val != other.val {
            return false;
        }

        if self.row == other.row {
            return true;
        }

        if self.col == other.col {
            return true;
        }

        let minigrid = (self.row / 3) * 3 + (self.col / 3);
        let other_minigrid = (other.row / 3) * 3 + (other.col / 3);

        minigrid == other_minigrid
    }

    pub fn same_cell(&self, other: &CellCandidate) -> bool {
        self.row == other.row && self.col == other.col
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

    pub fn can_see(&self, other: &Cell) -> bool {
        if self.row == other.row {
            return true;
        }

        if self.col == other.col {
            return true;
        }

        let minigrid = (self.row / 3) * 3 + (self.col / 3);
        let other_minigrid = (other.row / 3) * 3 + (other.col / 3);

        minigrid == other_minigrid
    }

    pub fn get_minigrid_n(&self) -> u32 {
        (self.get_row() / 3) * 3 + (self.get_col() / 3)
    }
}
