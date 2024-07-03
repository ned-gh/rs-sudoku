use itertools::Itertools;

use super::StrategyResult;
use crate::grid::{CellCandidate, Grid};

pub fn find_xyzwing(grid: &Grid) -> Option<StrategyResult> {
    let bivalue_cells = grid.get_nvalue_cells(2);
    let trivalue_cells = grid.get_nvalue_cells(3);

    for tv in trivalue_cells.iter() {
        for (bv1, bv2) in bivalue_cells.iter().tuple_combinations::<(_, _)>() {
            // check if the trivalue and two bivalues are of the form XYZ, XZ, YZ
            let xyz = tv.get_candidates();
            let xz = bv1.get_candidates();
            let yz = bv2.get_candidates();

            let has_three = xyz.union(xz).union(yz).len() == 3;
            let has_z = xz.intersection(yz).len() == 1;
            let has_xz = xyz.intersection(xz).len() == 2;
            let has_yz = xyz.intersection(yz).len() == 2;
            let one_common = xyz.intersection(xz).intersection(yz).len() == 1;

            if !(has_three && has_z && has_xz && has_yz && one_common) {
                continue;
            }

            // check that XZ and YZ can see XYZ
            if !(tv.can_see(bv1) && tv.can_see(bv2)) {
                continue;
            }

            let val = xz.intersection(yz).get_smallest();

            let sees_all = grid
                .get_cells_that_see(tv, false)
                .intersection(&grid.get_cells_that_see(bv1, false))
                .intersection(&grid.get_cells_that_see(bv2, false))
                .scan(val);

            if sees_all.is_empty() {
                continue;
            }

            return Some(StrategyResult::from(
                "XYZ-Wing",
                vec![],
                sees_all
                    .iter()
                    .map(|cell| CellCandidate::from_cell(cell, val))
                    .collect(),
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyzwing() {
        let bd =
            "072950681001780000500216000000428100200371006134569008800135064000897010015642890";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid properly: these eliminations are due to a pointing pair (7) and a naked
        // pair (2/7)
        grid.clear_candidate(&CellCandidate::from(3, 0, 7));
        grid.clear_candidate(&CellCandidate::from(3, 2, 7));
        grid.clear_candidate(&CellCandidate::from(1, 6, 2));
        grid.clear_candidate(&CellCandidate::from(2, 6, 7));
        grid.clear_candidate(&CellCandidate::from(7, 6, 2));

        let expected = vec![CellCandidate::from(8, 0, 3)];

        let xyzwing = find_xyzwing(&grid).unwrap();
        let to_place = xyzwing.get_to_place().clone();
        let to_eliminate = xyzwing.get_to_eliminate().clone();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
