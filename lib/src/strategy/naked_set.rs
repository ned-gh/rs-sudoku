use itertools::Itertools;

use super::{
    StrategyResult,
    highlight::{Highlight, HighlightColor},
};
use crate::grid::{CellCandidate, Cell, Grid, Region, UnitType};
use crate::util::BitSet;

use UnitType::{Col, MiniGrid, Row};

pub fn find_naked_set(grid: &Grid) -> Option<StrategyResult> {
    for size in 2..5 {
        for k in 0..9 {
            for unit_type in &[Row, Col, MiniGrid] {
                let cells = grid.get_unit(unit_type, k);

                for combination in cells.iter().cloned().combinations(size) {
                    let mut unique_candidates = BitSet::new();

                    for cell in combination.iter() {
                        unique_candidates.extend(cell.get_candidates());
                    }

                    if unique_candidates.len() != size as u32 {
                        continue;
                    }

                    let other = cells
                        .difference(&Region::from(&combination))
                        .scan_multiple(&unique_candidates);

                    if other.is_empty() {
                        continue;
                    }

                    let mut to_eliminate = vec![];

                    for cell in other.iter() {
                        for val in unique_candidates.iter() {
                            if cell.get_candidates().contains(val) {
                                to_eliminate.push(CellCandidate::from_cell(cell, val));
                            }
                        }
                    }

                    let highlights = make_highlights(&combination, &to_eliminate);

                    return Some(StrategyResult::from(
                        "Naked Set",
                        vec![],
                        to_eliminate,
                        highlights,
                    ));
                }
            }
        }
    }

    None
}

fn make_highlights(combination: &Vec<Cell>, to_eliminate: &Vec<CellCandidate>) -> Vec<Highlight> {
    let mut highlights = vec![];

    for cell in combination.iter() {
        for val in cell.get_candidates().iter() {
            highlights.push(Highlight::new_candidate_hl(
                &CellCandidate::from_cell(cell, val),
                HighlightColor::NoteFg,
                HighlightColor::NoteBg,
            ));
        }
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
    fn test_naked_sets() {
        let bd =
            "300009000000001020056002790003000048000040107009000000000000000080360200500820400";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(8, 1, 1),
            CellCandidate::from(8, 1, 7),
            CellCandidate::from(8, 7, 1),
            CellCandidate::from(8, 7, 7),
            CellCandidate::from(8, 8, 1),
        ];
        expected.sort();

        let naked_set = find_naked_set(&grid).unwrap();
        let mut to_place = naked_set.get_to_place().clone();
        let mut to_eliminate = naked_set.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
