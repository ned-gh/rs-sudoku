use crate::grid::{Grid, CellCandidate};
use super::StrategyResult;

pub fn find_naked_singles(grid: &Grid) -> Option<StrategyResult> {
    let mut singles = vec![];

    for r in 0..9 {
        for c in 0..9 {
            let candidates = grid.get_candidates(r, c);

            if candidates.len() == 1 {
                singles.push(CellCandidate::from(r, c, candidates.get_smallest()));
            }
        }
    }

    if singles.is_empty() {
        None
    } else {
        Some(StrategyResult::from(singles, vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naked_singles() {
        let bd = "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![
            CellCandidate::from(5, 2, 1),
        ];

        let result = find_naked_singles(&grid).unwrap();
        let to_place = result.get_to_place().clone();
        let to_eliminate = result.get_to_eliminate().clone();

        assert_eq!(expected, to_place);
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
