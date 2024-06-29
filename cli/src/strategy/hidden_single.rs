use std::collections::HashSet;

use super::{Strategy, StrategyResult};
use crate::grid::{CellCandidate, Grid, UnitType};

use UnitType::{Col, MiniGrid, Row};

pub struct HiddenSingles {
    result: StrategyResult,
}

impl HiddenSingles {
    fn from(result: StrategyResult) -> HiddenSingles {
        HiddenSingles { result }
    }
}

impl Strategy for HiddenSingles {
    fn find(grid: &Grid) -> Option<Self> {
        let mut singles = HashSet::new();

        for val in 1..10 {
            for k in 0..9 {
                for unit_type in &[Row, Col, MiniGrid] {
                    let cells = grid.get_unit(unit_type, k).scan(val);
                    if cells.len() == 1 {
                        let cell = cells.get_single();
                        singles.insert(CellCandidate::from_cell(&cell, val));
                    }
                }
            }
        }

        if singles.is_empty() {
            None
        } else {
            Some(HiddenSingles::from(StrategyResult::from(
                singles.into_iter().collect(),
                vec![],
            )))
        }
    }

    fn get_result(&self) -> &StrategyResult {
        &self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hidden_singles() {
        let bd =
            "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![CellCandidate::from(4, 8, 1), CellCandidate::from(4, 0, 7)];
        expected.sort();

        let singles = HiddenSingles::find(&grid).unwrap();
        let result = singles.get_result();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(expected, to_place);
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
