use std::collections::{HashSet, hash_set};

use super::cell::CellCandidate;


pub struct Region {
    values: HashSet<CellCandidate>,
}

impl Region {
    pub fn new() -> Region {
        Region { values: HashSet::new() }
    }

    pub fn from(values: HashSet<CellCandidate>) -> Region {
        Region { values }
    }

    pub fn insert(&mut self, cell_candidate: CellCandidate) {
        self.values.insert(cell_candidate);
    }

    pub fn intersect(&self, other: &Region) -> Region {
        Region::from(self.values.intersection(&other.values).cloned().collect())
    }

    pub fn union(&self, other: &Region) -> Region {
        Region::from(self.values.union(&other.values).cloned().collect())
    }

    pub fn iter(&self) -> hash_set::Iter<CellCandidate> {
        self.values.iter()
    }

    pub fn into_iter(self) -> hash_set::IntoIter<CellCandidate> {
        self.values.into_iter()
    }
}
