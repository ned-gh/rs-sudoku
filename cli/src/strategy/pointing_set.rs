use std::collections::HashSet;

use super::{Strategy, StrategyResult};
use crate::grid::{CellCandidate, Grid, Unit, UnitType};

pub struct PointingSets {
    result: StrategyResult,
}

impl PointingSets {
    fn from(result: StrategyResult) -> PointingSets {
        PointingSets { result }
    }
}

impl Strategy for PointingSets {
    fn find(grid: &Grid) -> Option<Self> {
        for val in 1..10 {
            for minigrid_n in 0..9 {
                let cells = grid.get_unit(&UnitType::MiniGrid, minigrid_n).scan(val);

                if !(cells.len() == 2 || cells.len() == 3) {
                    continue;
                }

                let Some(line) = cells.all_in_line() else {
                    continue;
                };

                let other = match &line {
                    Unit::Row(n) => grid.get_unit(&UnitType::Row, *n),
                    Unit::Col(n) => grid.get_unit(&UnitType::Col, *n),
                    _ => unreachable!(),
                }
                .difference(&cells)
                .scan(val);

                if other.is_empty() {
                    continue;
                }

                return Some(PointingSets::from(StrategyResult::from(
                    vec![],
                    other
                        .iter()
                        .map(|cell| CellCandidate::from_cell(cell, val))
                        .collect(),
                )));
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
    fn test_find_pointing_sets() {
        let bd =
            "300009000000001020056402790003200948005940107009000002000190000080360200501827400";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![CellCandidate::from(5, 4, 3)];

        let pointing_sets = PointingSets::find(&grid).unwrap();
        let result = pointing_sets.get_result();
        let to_place = result.get_to_place().clone();
        let to_eliminate = result.get_to_eliminate().clone();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
