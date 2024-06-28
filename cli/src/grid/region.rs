use std::collections::{HashSet, hash_set};

use super::cell::Cell;


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

    pub fn iter(&self) -> hash_set::Iter<Cell> {
        self.values.iter()
    }
}
