use itertools::Itertools;
use std::collections::HashSet;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, Region, UnitType};
use crate::util::BitSet;

pub fn find_naked_sets(grid: &Grid) -> Option<StrategyResult> {
    let mut to_eliminate = HashSet::new();

    for size in 2..5 {
        for k in 0..9 {
            for unit_type in &[UnitType::Row, UnitType::Col, UnitType::MiniGrid] {
                let cells = grid.get_unit(unit_type, k);

                for combination in cells.iter().cloned().combinations(size) {
                    let mut unique_candidates = BitSet::new();

                    for cell in combination.iter() {
                        unique_candidates.extend(&cell.get_candidates());
                    }

                    if unique_candidates.len() != size as u32 {
                        continue;
                    }

                    let other = cells.difference(&Region::from_vec(&combination));

                    let mut to_elim_in_other = vec![];
                    for cell in other.iter() {
                        for val in unique_candidates.iter() {
                            if cell.get_candidates().contains(val) {
                                to_elim_in_other.push(CellCandidate::from_cell(cell, val))
                            }
                        }
                    }

                    if to_elim_in_other.is_empty() {
                        continue;
                    }

                    to_eliminate.extend(to_elim_in_other.into_iter());
                }
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
    fn test_naked_sets() {
        let bd =
            "300009000000001020056002790003000048000040107009000000000000000080360200500820400";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(3, 5, 5),
            CellCandidate::from(3, 5, 7),
            CellCandidate::from(3, 6, 5),
            CellCandidate::from(3, 6, 6),
            CellCandidate::from(4, 5, 5),
            CellCandidate::from(4, 5, 6),
            CellCandidate::from(5, 5, 5),
            CellCandidate::from(5, 5, 6),
            CellCandidate::from(5, 5, 7),
            CellCandidate::from(5, 8, 3),
            CellCandidate::from(5, 8, 5),
            CellCandidate::from(5, 8, 6),
            CellCandidate::from(6, 3, 4),
            CellCandidate::from(6, 3, 5),
            CellCandidate::from(6, 3, 7),
            CellCandidate::from(6, 4, 5),
            CellCandidate::from(6, 4, 7),
            CellCandidate::from(8, 1, 1),
            CellCandidate::from(8, 1, 7),
            CellCandidate::from(8, 7, 1),
            CellCandidate::from(8, 7, 7),
            CellCandidate::from(8, 8, 1),
        ];
        expected.sort();

        let result = find_naked_sets(&grid).unwrap();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
