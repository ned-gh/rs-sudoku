use itertools::Itertools;

use super::{
    StrategyResult,
    highlight::{Highlight, HighlightColor},
};
use crate::grid::{CellCandidate, Grid, Region, UnitType};

use UnitType::{Col, Row};

pub fn find_xwing(grid: &Grid) -> Option<StrategyResult> {
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

                if line.len() == 2 {
                    lines.push(line);
                }
            }

            for (l1, l2) in lines.iter().tuple_combinations::<(_, _)>() {
                let cells = l1.union(l2);
                let (row_span, col_span) = cells.span();

                let opposite_span = match opposite {
                    Row => row_span,
                    Col => col_span,
                    _ => unreachable!(),
                };

                if opposite_span.len() != 2 {
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

                let to_eliminate = other
                    .iter()
                    .map(|cell| CellCandidate::from_cell(cell, val))
                    .collect();

                let highlights = make_highlights(&cells, val, &to_eliminate);

                return Some(StrategyResult::from(
                    "X-Wing",
                    vec![],
                    to_eliminate,
                    highlights,
                ));
            }
        }
    }

    None
}

fn make_highlights(cells: &Region, val: u32, to_eliminate: &Vec<CellCandidate>) -> Vec<Highlight> {
    let mut highlights = vec![];

    for cell in cells.iter() {
        highlights.push(Highlight::new_cell_hl(
            cell.get_row(),
            cell.get_col(),
            HighlightColor::Orange,
        ));

        highlights.push(Highlight::new_candidate_hl(
            &CellCandidate::from_cell(cell, val),
            HighlightColor::NoteFg,
            HighlightColor::NoteBg,
        ));
    }

    for cell_candidate in to_eliminate.iter() {
        highlights.push(Highlight::new_candidate_hl(
            cell_candidate,
            HighlightColor::ElimFg,
            HighlightColor::ElimBg,
        ));
    }

    highlights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xwing_rows() {
        let bd =
            "000910040060007508000000201090005100000030000003800070607000000584200010030086000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![
            CellCandidate::from(3, 8, 6),
            CellCandidate::from(4, 6, 6),
            CellCandidate::from(4, 8, 6),
            CellCandidate::from(5, 6, 6),
            CellCandidate::from(5, 8, 6),
        ];

        let xwing = find_xwing(&grid).unwrap();
        let to_place = xwing.get_to_place().clone();
        let mut to_eliminate = xwing.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }

    #[test]
    fn test_find_xwing_cols() {
        let bd =
            "300910040060307598000650231090005103000030803003860070607000380584203010030086000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![CellCandidate::from(8, 6, 9), CellCandidate::from(8, 8, 9)];

        let xwing = find_xwing(&grid).unwrap();
        let to_place = xwing.get_to_place().clone();
        let mut to_eliminate = xwing.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
