use super::StrategyResult;
use crate::grid::{CellCandidate, Grid};

pub fn find_naked_single(grid: &Grid) -> Option<StrategyResult> {
    for r in 0..9 {
        for c in 0..9 {
            let candidates = grid.get_candidates(r, c);

            if candidates.len() == 1 {
                return Some(StrategyResult::from(
                    vec![CellCandidate::from(r, c, candidates.get_smallest())],
                    vec![],
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
    fn test_naked_singles() {
        let bd =
            "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![CellCandidate::from(5, 2, 1)];

        let single = find_naked_single(&grid).unwrap();
        let to_place = single.get_to_place().clone();
        let to_eliminate = single.get_to_eliminate().clone();

        assert_eq!(expected, to_place);
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
