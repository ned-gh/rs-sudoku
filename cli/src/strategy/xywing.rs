use itertools::Itertools;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid};

pub fn find_xywing(grid: &Grid) -> Option<StrategyResult> {
    let bivalue_cells = grid.get_nvalue_cells(2);

    // We are checking for three bivalue cells that have the form AB, BC, AC.
    // By using permutations, we guarantee that if there is three such bivalue cells then we will
    // see them in that order.
    for bivalues in bivalue_cells.iter().permutations(3) {
        let ab = bivalues[0];
        let bc = bivalues[1];
        let ac = bivalues[2];

        let ab_cands = ab.get_candidates();
        let bc_cands = bc.get_candidates();
        let ac_cands = ac.get_candidates();

        // check if ab, bc, ac are actually of the form AB, BC, AC
        let has_three = ab_cands.union(bc_cands).union(ac_cands).len() == 3;
        let has_b = ab_cands.intersection(bc_cands).len() == 1;
        let has_a = ab_cands.intersection(ac_cands).len() == 1;
        let has_c = bc_cands.intersection(ac_cands).len() == 1;
        let no_common = ab_cands.intersection(bc_cands).intersection(ac_cands).len() == 0;

        if !has_three || !has_b || !has_a || !has_c || !no_common {
            continue;
        }

        // check that BC and AC can see AB
        if !bc.can_see(ab) || !ac.can_see(ab) {
            continue;
        }

        // val = C
        let val = bc_cands.intersection(ac_cands).get_smallest();

        let sees_both = grid
            .get_cells_that_see(bc, false)
            .intersection(&grid.get_cells_that_see(ac, false))
            .scan(val);

        if sees_both.is_empty() {
            continue;
        }

        return Some(StrategyResult::from(
            vec![],
            sees_both
                .iter()
                .map(|cell| CellCandidate::from_cell(cell, val))
                .collect(),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xywing() {
        let bd =
            "357000400062070930094203700235847619716002843948136200029715380573028190081300500";
        let grid = Grid::from_str(bd).unwrap();

        let expected = vec![CellCandidate::from(0, 5, 9)];

        let xywing = find_xywing(&grid).unwrap();
        let to_place = xywing.get_to_place().clone();
        let to_eliminate = xywing.get_to_eliminate().clone();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
