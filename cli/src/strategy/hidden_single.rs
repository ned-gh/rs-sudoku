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
        for val in 1..10 {
            for k in 0..9 {
                for unit_type in &[Row, Col, MiniGrid] {
                    let cells = grid.get_unit(unit_type, k).scan(val);
                    if cells.len() == 1 {
                        let cell = cells.get_single();

                        return Some(HiddenSingles::from(StrategyResult::from(
                            vec![CellCandidate::from_cell(&cell, val)],
                            vec![],
                        )));
                    }
                }
            }
        }

        None
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

        let expected = vec![CellCandidate::from(4, 8, 1)];

        let singles = HiddenSingles::find(&grid).unwrap();
        let result = singles.get_result();
        let to_place = result.get_to_place().clone();
        let to_eliminate = result.get_to_eliminate().clone();

        assert_eq!(expected, to_place);
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
