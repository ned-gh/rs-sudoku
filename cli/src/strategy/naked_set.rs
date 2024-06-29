use itertools::Itertools;
use std::collections::HashSet;

use super::{Strategy, StrategyResult};
use crate::grid::{CellCandidate, Grid, Region, UnitType};
use crate::util::BitSet;

use UnitType::{Col, MiniGrid, Row};

pub struct NakedSets {
    result: StrategyResult,
}

impl NakedSets {
    fn from(result: StrategyResult) -> NakedSets {
        NakedSets { result }
    }
}

impl Strategy for NakedSets {
    fn find(grid: &Grid) -> Option<Self> {
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
                            .difference(&Region::from_vec(&combination))
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

                        return Some(NakedSets::from(StrategyResult::from(vec![], to_eliminate)));
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

        let hidden_sets = NakedSets::find(&grid).unwrap();
        let result = hidden_sets.get_result();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
