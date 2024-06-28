use std::collections::HashSet;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Unit, UnitType};

pub fn find_locked_candidates(grid: &Grid) -> Option<StrategyResult> {
    let mut to_eliminate = HashSet::new();

    for val in 1..10 {
        for k in 0..9 {
            for unit_type in &[UnitType::Row, UnitType::Col] {
                let cells = grid.get_unit(unit_type, k).scan(val);

                if cells.is_empty() {
                    continue;
                }

                let Some(minigrid) = cells.all_in_minigrid() else {
                    continue;
                };

                let other = match minigrid {
                    Unit::MiniGrid(n) => grid.get_unit(&UnitType::MiniGrid, n),
                    _ => unreachable!(),
                }
                .difference(&cells)
                .scan(val);

                if other.is_empty() {
                    continue;
                }

                to_eliminate.extend(other.iter().map(|cell| CellCandidate::from_cell(cell, val)));
            }
        }
    }

    if to_eliminate.is_empty() {
        None
    } else {
        Some(StrategyResult::from(
            vec![],
            to_eliminate.into_iter().collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locked_candidates() {
        let bd =
            "000910040060007508000000201090005100000030000003800070607000000584200010030086000";
        let mut grid = Grid::from_str(bd).unwrap();

        // remove 6 candidate from (row, col) = (2, 7) to get correct grid state
        grid.clear_candidate(2, 7, 6);

        let mut expected = vec![
            CellCandidate::from(0, 0, 7),
            CellCandidate::from(0, 1, 7),
            CellCandidate::from(2, 1, 5),
            CellCandidate::from(2, 2, 5),
            CellCandidate::from(4, 6, 6),
            CellCandidate::from(5, 6, 6),
            CellCandidate::from(3, 8, 6),
            CellCandidate::from(4, 8, 6),
            CellCandidate::from(5, 8, 6),
        ];
        expected.sort();

        let result = find_locked_candidates(&grid).unwrap();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
