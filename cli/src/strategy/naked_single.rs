use crate::grid::{Grid, CellCandidate};

pub fn find_naked_singles(grid: &Grid) -> Option<Vec<CellCandidate>> {
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
        Some(singles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naked_singles() {
        let bd = "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = Some(vec![
            CellCandidate::from(5, 2, 1),
        ]);

        assert_eq!(expected, find_naked_singles(&grid));
    }
}
