use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Unit, UnitType};

pub fn find_pointing_set(grid: &Grid) -> Option<StrategyResult> {
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

            return Some(StrategyResult::from(
                "Pointing Set",
                vec![],
                other
                    .iter()
                    .map(|cell| CellCandidate::from_cell(cell, val))
                    .collect(),
            ));
        }
    }

    None
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

        let pointing_set = find_pointing_set(&grid).unwrap();
        let to_place = pointing_set.get_to_place().clone();
        let to_eliminate = pointing_set.get_to_eliminate().clone();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
