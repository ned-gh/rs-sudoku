use std::collections::{HashSet, hash_set};

use crate::util::BitSet;
use super::{Cell, Unit};


#[derive(Debug)]
pub struct Region {
    values: HashSet<Cell>,
}

impl Region {
    pub fn new() -> Region {
        Region { values: HashSet::new() }
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
            Some(Unit::Row(row_span.get_smallest()))
        } else if col_span.len() == 1 {
            Some(Unit::Col(col_span.get_smallest()))
        } else {
            None
        }
    }

    fn span(&self) -> (BitSet, BitSet) {
        let mut row_span = BitSet::new();
        let mut col_span = BitSet::new();

        for cell in self.iter() {
            row_span.insert(cell.get_row());
            col_span.insert(cell.get_col());
        }

        (row_span, col_span)
    }

    pub fn get_single(&self) -> Cell{
        self.iter().next().unwrap().clone()
    }

    pub fn insert(&mut self, cell: Cell) {
        self.values.insert(cell);
    }

    pub fn intersection(&self, other: &Region) -> Region {
        Region { values: self.values.intersection(&other.values).cloned().collect() }
    }

    pub fn union(&self, other: &Region) -> Region {
        Region { values: self.values.union(&other.values).cloned().collect() }
    }

    pub fn difference(&self, other: &Region) -> Region {
        Region { values: self.values.difference(&other.values).cloned().collect() }
    }

    pub fn iter(&self) -> hash_set::Iter<Cell> {
        self.values.iter()
    }
}
