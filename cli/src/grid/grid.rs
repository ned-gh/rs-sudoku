use super::Cell;
use super::Region;
use super::UnitType;
use crate::util::BitSet;

pub struct Grid {
    placed: Vec<u32>,
    candidates: Vec<BitSet>,
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

        let mut grid = Grid {
            placed,
            candidates: vec![],
        };
        grid.autofill();

        Ok(grid)
    }

    fn get_placed(&self, row: u32, col: u32) -> Option<u32> {
        match self.placed[index(row, col)] {
            0 => None,
            n => Some(n),
        }
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
                    let minigrid = self.get_placed_in_minigrid(minigrid_n(r, c));

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
                    region.insert(Cell::from(num, c, &self.candidates[index(num, c)]));
                }
            }

            UnitType::Col => {
                for r in 0..9 {
                    region.insert(Cell::from(r, num, &self.candidates[index(r, num)]));
                }
            }

            UnitType::MiniGrid => {
                let (cr, cc) = minigrid_corners(num);

                for r in 0..3 {
                    for c in 0..3 {
                        let row = cr + r;
                        let col = cc + c;

                        region.insert(Cell::from(row, col, &self.candidates[index(row, col)]));
                    }
                }
            }
        }

        region
    }

    pub fn get_candidates(&self, row: u32, col: u32) -> &BitSet {
        self.candidates.get(index(row, col)).unwrap()
    }
}

pub fn index(row: u32, col: u32) -> usize {
    (9 * row + col) as usize
}

pub fn minigrid_n(row: u32, col: u32) -> u32 {
    (row / 3) * 3 + (col / 3)
}

pub fn minigrid_corners(box_n: u32) -> (u32, u32) {
    let corner_row = (box_n / 3) * 3;
    let corner_col = (box_n % 3) * 3;

    (corner_row, corner_col)
}
