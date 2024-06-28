use std::collections::HashSet;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Unit, UnitType};

pub fn find_pointing_sets(grid: &Grid) -> Option<StrategyResult> {
    let mut to_eliminate = HashSet::new();

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

            to_eliminate.extend(other.iter().map(|cell| CellCandidate::from_cell(cell, val)));
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
    fn test_find_pointing_sets() {
        let bd =
            "300009000000001020056402790003200948005940107009000002000190000080360200501827400";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(5, 4, 3),
            CellCandidate::from(5, 3, 5),
            CellCandidate::from(5, 4, 5),
            CellCandidate::from(5, 5, 5),
            CellCandidate::from(3, 5, 5),
            CellCandidate::from(1, 0, 8),
            CellCandidate::from(2, 0, 8),
            CellCandidate::from(5, 4, 8),
            CellCandidate::from(5, 3, 6),
        ];
        expected.sort();

        let result = find_pointing_sets(&grid).unwrap();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
