use crate::grid::CellCandidate;

#[derive(Debug)]
pub struct StrategyResult {
    name: String,
    to_place: Vec<CellCandidate>,
    to_eliminate: Vec<CellCandidate>,
}

impl StrategyResult {
    pub fn from(
        name: &str,
        to_place: Vec<CellCandidate>,
        to_eliminate: Vec<CellCandidate>,
    ) -> StrategyResult {
        StrategyResult {
            name: name.to_string(),
            to_place,
            to_eliminate,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_to_place(&self) -> &Vec<CellCandidate> {
        &self.to_place
    }

    pub fn get_to_eliminate(&self) -> &Vec<CellCandidate> {
        &self.to_eliminate
    }
}
