use std::collections::HashSet;

use super::cell::CellCandidate;


pub struct Grid {
    placed: Vec<u32>,
    candidates: Vec<HashSet<u32>>,
}

#[derive(Debug)]
pub enum GridError {
    InvalidCharacter(char),
    InvalidGridSize(usize),
}

impl Grid {
    pub fn from_str(bd: &str) -> Result<Grid, GridError> {
        if bd.len() != 81 {
            return Err(GridError::InvalidGridSize(bd.len()));
        }

        let mut placed = vec![];

        for c in bd.chars() {
            match c.to_digit(10) {
                Some(n) => placed.push(n),
                None => return Err(GridError::InvalidCharacter(c)),
            }
        }

        let mut grid = Grid { placed, candidates: vec![] };
        grid.autofill();

        Ok(grid)
    }

    fn get_placed_in_row(&self, row: u32) -> Vec<u32> {
        let mut nums = vec![];

        for c in 0..9 {
            match self.placed.get(index(row, c)).unwrap() {
                0 => (),
                n => nums.push(*n),
            };
        }

        nums
    }

    fn get_placed_in_col(&self, col: u32) -> Vec<u32> {
        let mut nums = vec![];

        for r in 0..9 {
            match self.placed.get(index(r, col)).unwrap() {
                0 => (),
                n => nums.push(*n),
            };
        }

        nums
    }

    fn get_placed_in_box(&self, box_n: u32) -> Vec<u32> {
        let (cr, cc) = box_corners(box_n);

        let mut nums = vec![];

        for r in 0..3 {
            for c in 0..3 {
                match self.placed.get(index(cr + r, cc + c)).unwrap() {
                    0 => (),
                    n => nums.push(*n),
                };
            }
        }

        nums
    }

    fn autofill(&mut self) {
        for r in 0..9 {
            for c in 0..9 {
                let mut cell_cands = HashSet::new();

                if *self.placed.get(index(r, c)).unwrap() == 0 {
                    let row = self.get_placed_in_row(r);
                    let col = self.get_placed_in_col(c);
                    let box_ = self.get_placed_in_box(box_n(r, c));

                    for n in 1..10 {
                        if !(row.contains(&n) || col.contains(&n) || box_.contains(&n)) {
                            cell_cands.insert(n);
                        }
                    }
                }

                self.candidates.push(cell_cands);
            }
        }
    }

    pub fn scan_row(&self, row: u32, val: u32) -> Vec<CellCandidate> {
        let mut res = vec![];

        for c in 0..9 {
            for &n in self.candidates.get(index(row, c)).unwrap().iter() {
                if n == val {
                    res.push(CellCandidate::new(row,  c, n));
                }
            }
        }

        res
    }

    pub fn scan_col(&self, col: u32, val: u32) -> Vec<CellCandidate> {
        let mut res = vec![];

        for r in 0..9 {
            for &n in self.candidates.get(index(r, col)).unwrap().iter() {
                if n == val {
                    res.push(CellCandidate::new(r, col, n));
                }
            }
        }

        res
    }

    pub fn scan_box_n(&self, box_n: u32, val: u32) -> Vec<CellCandidate> {
        let mut res = vec![];

        let (cr, cc) = box_corners(box_n);

        for r in 0..3 {
            for c in 0..3 {
                for &n in self.candidates.get(index(cr + r, cc + c)).unwrap().iter() {
                    if n == val {
                        res.push(CellCandidate::new(cr + r, cc + c, n));
                    }
                }
            }
        }

        res
    }

    pub fn get_candidates(&self, row: u32, col: u32) -> &HashSet<u32> {
        self.candidates.get(index(row, col)).unwrap()
    }
}

pub fn index(row: u32, col: u32) -> usize {
    (9 * row + col) as usize
}

pub fn box_n(row: u32, col: u32) -> u32 {
    (row / 3) * 3 + (col / 3)
}

pub fn box_corners(box_n: u32) -> (u32, u32) {
    let corner_row = (box_n / 3) * 3;
    let corner_col = (box_n % 3) * 3;

    (corner_row, corner_col)
}
