use std::collections::{hash_set, HashSet};

use super::{Cell, Unit};
use crate::util::BitSet;

#[derive(Debug)]
pub struct Region {
    values: HashSet<Cell>,
}

impl Region {
    pub fn new() -> Region {
        Region {
            values: HashSet::new(),
        }
    }

    pub fn from_vec(values_vec: &Vec<Cell>) -> Region {
        let mut values = HashSet::new();
        values.extend(values_vec.iter().cloned());

        Region { values }
    }

    pub fn len(&self) -> u32 {
        self.values.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn scan(&self, val: u32) -> Region {
        let mut values = HashSet::new();

        for cell in self.values.iter() {
            if cell.get_candidates().contains(val) {
                values.insert(cell.clone());
            }
        }

        Region { values }
    }

    pub fn all_in_line(&self) -> Option<Unit> {
        let (row_span, col_span) = self.span();

        if row_span.len() == 1 {
            Some(Unit::Row(*row_span.iter().next().unwrap()))
        } else if col_span.len() == 1 {
            Some(Unit::Col(*col_span.iter().next().unwrap()))
        } else {
            None
        }
    }

    pub fn all_in_minigrid(&self) -> Option<Unit> {
        let mut min_cr = 8;
        let mut min_cc = 8;
        let mut max_cr = 0;
        let mut max_cc = 0;

        for cell in self.iter() {
            let cr = cell.get_row() / 3;
            let cc = cell.get_col() / 3;

            if cr < min_cr {
                min_cr = cr;
            }

            if cc < min_cc {
                min_cc = cc;
            }

            if cr > max_cr {
                max_cr = cr;
            }

            if cc > max_cc {
                max_cc = cc;
            }
        }

        if min_cr != max_cr || min_cc != max_cc {
            None
        } else {
            Some(Unit::MiniGrid(min_cr * 3 + min_cc))
        }
    }

    fn span(&self) -> (HashSet<u32>, HashSet<u32>) {
        let mut row_span = HashSet::new();
        let mut col_span = HashSet::new();

        for cell in self.iter() {
            row_span.insert(cell.get_row());
            col_span.insert(cell.get_col());
        }

        (row_span, col_span)
    }

    pub fn get_single(&self) -> Cell {
        self.iter().next().unwrap().clone()
    }

    pub fn insert(&mut self, cell: Cell) {
        self.values.insert(cell);
    }

    pub fn intersection(&self, other: &Region) -> Region {
        Region {
            values: self.values.intersection(&other.values).cloned().collect(),
        }
    }

    pub fn union(&self, other: &Region) -> Region {
        Region {
            values: self.values.union(&other.values).cloned().collect(),
        }
    }

    pub fn difference(&self, other: &Region) -> Region {
        Region {
            values: self.values.difference(&other.values).cloned().collect(),
        }
    }

    pub fn iter(&self) -> hash_set::Iter<Cell> {
        self.values.iter()
    }
}
