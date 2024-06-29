use crate::grid::{CellCandidate, Grid};

#[derive(Debug)]
pub struct StrategyResult {
    to_place: Vec<CellCandidate>,
    to_eliminate: Vec<CellCandidate>,
}

impl StrategyResult {
    pub fn from(to_place: Vec<CellCandidate>, to_eliminate: Vec<CellCandidate>) -> StrategyResult {
        StrategyResult {
            to_place,
            to_eliminate,
        }
    }

    pub fn get_to_place(&self) -> &Vec<CellCandidate> {
        &self.to_place
    }

    pub fn get_to_eliminate(&self) -> &Vec<CellCandidate> {
        &self.to_eliminate
    }
}
