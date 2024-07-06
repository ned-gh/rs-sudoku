use serde::Serialize;

use super::highlight::Highlight;
use crate::grid::CellCandidate;

#[derive(Debug, Serialize)]
pub struct StrategyResult {
    name: String,
    to_place: Vec<CellCandidate>,
    to_eliminate: Vec<CellCandidate>,
    highlights: Vec<Highlight>,
}

impl StrategyResult {
    pub fn from(
        name: &str,
        to_place: Vec<CellCandidate>,
        to_eliminate: Vec<CellCandidate>,
        highlights: Vec<Highlight>,
    ) -> StrategyResult {
        StrategyResult {
            name: name.to_string(),
            to_place,
            to_eliminate,
            highlights,
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
