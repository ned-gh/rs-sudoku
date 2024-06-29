use itertools::Itertools;
use std::collections::HashSet;

use super::{Strategy, StrategyResult};
use crate::grid::{CellCandidate, Grid, UnitType};
use crate::util::BitSet;

use UnitType::{Col, MiniGrid, Row};

pub struct HiddenSets {
    result: StrategyResult,
}

impl HiddenSets {
    fn from(result: StrategyResult) -> HiddenSets {
        HiddenSets { result }
    }
}

impl Strategy for HiddenSets {
    fn find(grid: &Grid) -> Option<Self> {
        for size in 2..5 {
            for k in 0..9 {
                for unit_type in &[Row, Col, MiniGrid] {
                    let unit_cells = grid.get_unit(unit_type, k);
                    let candidate_span = unit_cells.candidate_span();

                    for candidate_combination in candidate_span.iter().combinations(size as usize) {
                        let candidate_bitset = BitSet::from(&candidate_combination);

                        let cells = unit_cells.scan_multiple(&candidate_bitset);

                        if cells.len() != size {
                            continue;
                        }

                        let mut to_eliminate = vec![];

                        for cell in cells.iter() {
                            let elim_candidates =
                                cell.get_candidates().difference(&candidate_bitset);

                            for val in elim_candidates.iter() {
                                to_eliminate.push(CellCandidate::from_cell(cell, val));
                            }
                        }

                        if !to_eliminate.is_empty() {
                            return Some(HiddenSets::from(StrategyResult::from(
                                vec![],
                                to_eliminate,
                            )));
                        }
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
    fn test_hidden_sets() {
        let bd =
            "300009000000001020056402790003200948005940107009000002000190000080360200501827400";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(4, 5, 6),
            CellCandidate::from(5, 5, 5),
            CellCandidate::from(5, 5, 6),
        ];
        expected.sort();

        let hidden_sets = HiddenSets::find(&grid).unwrap();
        let result = hidden_sets.get_result();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        for cell in to_eliminate.iter() {
            println!("{:?}", cell);
        }

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
