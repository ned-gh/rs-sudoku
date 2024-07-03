use itertools::Itertools;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Region, UnitType};

use UnitType::{Col, Row};

pub fn find_swordfish(grid: &Grid) -> Option<StrategyResult> {
    for val in 1..10 {
        for unit_type in &[Row, Col] {
            let opposite = match unit_type {
                Row => Col,
                Col => Row,
                _ => unreachable!(),
            };

            let mut lines = vec![];

            for k in 0..9 {
                let line = grid.get_unit(unit_type, k).scan(val);

                if line.len() == 2 || line.len() == 3 {
                    lines.push(line);
                }
            }

            for (l1, l2, l3) in lines.iter().tuple_combinations::<(_, _, _)>() {
                let cells = l1.union(l2).union(l3);
                let (row_span, col_span) = cells.span();

                let opposite_span = match opposite {
                    Row => row_span,
                    Col => col_span,
                    _ => unreachable!(),
                };

                if opposite_span.len() != 3 {
                    continue;
                }

                let other = opposite_span
                    .iter()
                    .map(|n| grid.get_unit(&opposite, *n).scan(val))
                    .reduce(|acc, region| acc.union(&region))
                    .unwrap_or(Region::new())
                    .difference(&cells);

                if other.is_empty() {
                    continue;
                }

                return Some(StrategyResult::from(
                    "Swordfish",
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
    fn test_find_swordfish() {
        let bd =
            "200709006190000002080002030670503040409000305350904060060300090800090053900407001";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(4, 4, 2),
            CellCandidate::from(7, 2, 2),
            CellCandidate::from(8, 2, 2),
            CellCandidate::from(8, 4, 2),
            CellCandidate::from(7, 6, 2),
            CellCandidate::from(8, 6, 2),
        ];
        expected.sort();

        let swordfish = find_swordfish(&grid).unwrap();
        let to_place = swordfish.get_to_place().clone();
        let mut to_eliminate = swordfish.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
