use crate::grid::{Grid, CellCandidate};

pub fn find_naked_single(grid: &Grid) -> Option<Vec<CellCandidate>> {
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
