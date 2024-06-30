use std::fmt;

use super::{Cell, CellCandidate, Region, UnitType};
use crate::util::BitSet;

pub struct Grid {
    placed: Vec<u32>,
    candidates: Vec<BitSet>,
}

#[allow(dead_code)]
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

        let mut grid = Grid {
            placed,
            candidates: vec![],
        };
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

    fn get_placed_in_minigrid(&self, box_n: u32) -> Vec<u32> {
        let (cr, cc) = minigrid_corners(box_n);

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
                let mut cell_cands = BitSet::new();

                if *self.placed.get(index(r, c)).unwrap() == 0 {
                    let row = self.get_placed_in_row(r);
                    let col = self.get_placed_in_col(c);
                    let minigrid = self.get_placed_in_minigrid(get_minigrid_n_from_coords(r, c));

                    for n in 1..10 {
                        if !(row.contains(&n) || col.contains(&n) || minigrid.contains(&n)) {
                            cell_cands.insert(n);
                        }
                    }
                }

                self.candidates.push(cell_cands);
            }
        }
    }

    pub fn get_unit(&self, unit_type: &UnitType, num: u32) -> Region {
        let mut region = Region::new();

        match unit_type {
            UnitType::Row => {
                for c in 0..9 {
                    if self.placed[index(num, c)] != 0 {
                        continue;
                    }
                    region.insert(Cell::from(num, c, &self.candidates[index(num, c)]));
                }
            }

            UnitType::Col => {
                for r in 0..9 {
                    if self.placed[index(r, num)] != 0 {
                        continue;
                    }
                    region.insert(Cell::from(r, num, &self.candidates[index(r, num)]));
                }
            }

            UnitType::MiniGrid => {
                let (cr, cc) = minigrid_corners(num);

                for r in 0..3 {
                    for c in 0..3 {
                        let row = cr + r;
                        let col = cc + c;

                        if self.placed[index(row, col)] != 0 {
                            continue;
                        }

                        region.insert(Cell::from(row, col, &self.candidates[index(row, col)]));
                    }
                }
            }
        }

        region
    }

    pub fn get_unit_containing(&self, unit_type: &UnitType, cell: &Cell) -> Region {
        match unit_type {
            UnitType::Row => self.get_unit(unit_type, cell.get_row()),
            UnitType::Col => self.get_unit(unit_type, cell.get_col()),
            UnitType::MiniGrid => self.get_unit(unit_type, cell.get_minigrid_n()),
        }
    }

    pub fn get_candidates(&self, row: u32, col: u32) -> &BitSet {
        &self.candidates[index(row, col)]
    }

    pub fn clear_candidate(&mut self, cell_candidate: &CellCandidate) {
        let (row, col, val) = cell_candidate.as_tuple();
        self.candidates[index(row, col)].remove(val);
    }

    fn clear_row(&mut self, row: u32, val: u32) {
        for c in 0..9 {
            self.candidates[index(row, c)].remove(val);
        }
    }

    fn clear_col(&mut self, col: u32, val: u32) {
        for r in 0..9 {
            self.candidates[index(r, col)].remove(val);
        }
    }

    fn clear_minigrid(&mut self, row: u32, col: u32, val: u32) {
        let cr = (row / 3) * 3;
        let cc = (col / 3) * 3;

        for r in 0..3 {
            for c in 0..3 {
                self.candidates[index(cr + r, cc + c)].remove(val);
            }
        }
    }

    pub fn place(&mut self, cell_candidate: &CellCandidate) {
        let (r, c, val) = cell_candidate.as_tuple();

        self.placed[index(r, c)] = val;

        self.candidates[index(r, c)] = BitSet::new();

        self.clear_row(r, val);
        self.clear_col(c, val);
        self.clear_minigrid(r, c, val);
    }

    pub fn is_complete(&self) -> bool {
        for k in 0..9 {
            let row_vals = self.get_placed_in_row(k);
            if row_vals.len() != 9 || row_vals.contains(&0) {
                return false;
            }

            let col_vals = self.get_placed_in_col(k);
            if col_vals.len() != 9 || col_vals.contains(&0) {
                return false;
            }

            let minigrid_vals = self.get_placed_in_minigrid(k);
            if minigrid_vals.len() != 9 || col_vals.contains(&0) {
                return false;
            }
        }

        true
    }

    pub fn get_nvalue_cells(&self, n: u32) -> Region {
        let mut cells = Region::new();

        for r in 0..9 {
            for c in 0..9 {
                let cands = &self.candidates[index(r, c)];

                if cands.len() == n {
                    cells.insert(Cell::from(r, c, cands));
                }
            }
        }

        cells
    }

    pub fn get_cells_that_see(&self, cell: &Cell, include_cell: bool) -> Region {
        let row = cell.get_row();
        let col = cell.get_col();
        let minigrid = cell.get_minigrid_n();

        let mut cells = self
            .get_unit(&UnitType::Row, row)
            .union(&self.get_unit(&UnitType::Col, col))
            .union(&self.get_unit(&UnitType::MiniGrid, minigrid));

        if !include_cell {
            cells.remove(cell);
        }

        cells
    }
}

pub fn get_minigrid_n_from_coords(row: u32, col: u32) -> u32 {
    (row / 3) * 3 + (col / 3)
}

fn index(row: u32, col: u32) -> usize {
    (9 * row + col) as usize
}

fn minigrid_corners(box_n: u32) -> (u32, u32) {
    let corner_row = (box_n / 3) * 3;
    let corner_col = (box_n % 3) * 3;

    (corner_row, corner_col)
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = vec![];

        for r in 0..9 {
            let mut row = vec![];

            for c in 0..9 {
                let val = self.placed[index(r, c)];

                if val == 0 {
                    row.push(String::from(" "));
                } else {
                    row.push(val.to_string());
                }

                if (c + 1) % 3 == 0 && c < 8 {
                    row.push("|".to_string());
                }
            }

            s.push(row.join(" "));

            if (r + 1) % 3 == 0 && r < 8 {
                let hline = row
                    .iter()
                    .map(|ch| {
                        if ch != "|" {
                            "-".to_string()
                        } else {
                            "+".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("-");

                s.push(hline);
            }
        }

        write!(f, "{}", s.join("\n"))
    }
}
