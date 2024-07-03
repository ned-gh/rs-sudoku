use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Unit, UnitType};

pub fn find_locked_candidates(grid: &Grid) -> Option<StrategyResult> {
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

                return Some(StrategyResult::from(
                    "Locked Candidates",
                    vec![],
                    other
                        .iter()
                        .map(|cell| CellCandidate::from_cell(cell, val))
                        .collect(),
                ));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locked_candidates() {
        let bd =
            "000910040060007508000000201090005100000030000003800070607000000584200010030086000";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![CellCandidate::from(2, 1, 5), CellCandidate::from(2, 2, 5)];
        expected.sort();

        let locked_candidate = find_locked_candidates(&grid).unwrap();
        let mut to_place = locked_candidate.get_to_place().clone();
        let mut to_eliminate = locked_candidate.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
