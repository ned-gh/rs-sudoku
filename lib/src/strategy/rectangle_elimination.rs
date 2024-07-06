use itertools::Itertools;

use super::{
    highlight::{Highlight, HighlightColor},
    StrategyResult,
};
use crate::grid::{get_minigrid_n_from_coords, Cell, CellCandidate, Grid, Region, UnitType};

use UnitType::{Col, MiniGrid, Row};

pub fn find_rectangle_elimination(grid: &Grid) -> Option<StrategyResult> {
    for val in 1..10 {
        for unit_type in &[Row, Col] {
            let opposite = match unit_type {
                Row => Col,
                Col => Row,
                _ => unreachable!(),
            };

            let mut lines = vec![];

            for k in 0..9 {
                let line = grid.get_unit(unit_type, k).scan(val);

                if line.len() == 2 {
                    lines.push(line);
                }
            }

            for line in lines.iter() {
                let Some((cell1, cell2)) = line.iter().next_tuple() else {
                    continue;
                };

                if cell1.get_minigrid_n() == cell2.get_minigrid_n() {
                    continue;
                }

                let possible_wings = get_possible_wings(grid, cell1, cell2, val, &opposite);

                for wing in possible_wings.iter() {
                    let (hinge, other_wing) = if wing.can_see(cell1) {
                        (cell1, cell2)
                    } else {
                        (cell2, cell1)
                    };

                    if !wings_eliminate_minigrid(grid, wing, other_wing, val, &opposite) {
                        continue;
                    }

                    // if the hinge and wing are also strongly linked, then the hinge must contain
                    // val
                    let hinge_wing_unit = grid.get_unit_containing(&opposite, hinge).scan(val);

                    if hinge_wing_unit.len() == 2 {
                        let highlights = make_highlights(hinge, wing, other_wing, val, true);

                        return Some(StrategyResult::from(
                            "Rectangle Elimination (two strong links)",
                            vec![CellCandidate::from_cell(hinge, val)],
                            vec![],
                            highlights,
                        ));
                    } else {
                        let highlights = make_highlights(hinge, wing, other_wing, val, false);

                        return Some(StrategyResult::from(
                            "Rectangle Elimination",
                            vec![],
                            vec![CellCandidate::from_cell(wing, val)],
                            highlights,
                        ));
                    }
                }
            }
        }
    }

    None
}

// cells that are potential wings are those cells that are in the units in the opposite direction
// that contain cell1 or cell2, but are not in either of their minigrids
fn get_possible_wings(
    grid: &Grid,
    cell1: &Cell,
    cell2: &Cell,
    val: u32,
    opposite: &UnitType,
) -> Region {
    let minigrids = grid
        .get_unit(&MiniGrid, cell1.get_minigrid_n())
        .union(&grid.get_unit(&MiniGrid, cell2.get_minigrid_n()));

    match opposite {
        Row => grid
            .get_unit(opposite, cell1.get_row())
            .union(&grid.get_unit(opposite, cell2.get_row())),
        Col => grid
            .get_unit(opposite, cell1.get_col())
            .union(&grid.get_unit(opposite, cell2.get_col())),
        _ => unreachable!(),
    }
    .difference(&minigrids)
    .scan(val)
}

// check if the wings remove val as a candidate in all of a minigrid (the minigrid must contain val
// as a candidate in atleast one cell)
fn wings_eliminate_minigrid(
    grid: &Grid,
    wing: &Cell,
    other_wing: &Cell,
    val: u32,
    opposite: &UnitType,
) -> bool {
    let target_minigrid = match opposite {
        Row => get_minigrid_n_from_coords(other_wing.get_row(), wing.get_col()),
        Col => get_minigrid_n_from_coords(wing.get_row(), other_wing.get_col()),
        _ => unreachable!(),
    };

    let target_cells = grid.get_unit(&MiniGrid, target_minigrid).scan(val);

    if target_cells.is_empty() {
        return false;
    }

    target_cells.is_subset(
        &grid
            .get_cells_that_see(wing, true)
            .union(&grid.get_cells_that_see(other_wing, true)),
    )
}

fn make_highlights(
    hinge: &Cell,
    wing: &Cell,
    other_wing: &Cell,
    val: u32,
    place: bool,
) -> Vec<Highlight> {
    if place {
        vec![
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(hinge, val),
                HighlightColor::NoteFg,
                HighlightColor::NoteBg,
            ),
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(wing, val),
                HighlightColor::ElimFg,
                HighlightColor::ElimBg,
            ),
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(other_wing, val),
                HighlightColor::ElimFg,
                HighlightColor::ElimBg,
            ),
        ]
    } else {
        vec![
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(hinge, val),
                HighlightColor::NoteNegativeFg,
                HighlightColor::NoteNegativeBg,
            ),
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(wing, val),
                HighlightColor::ElimFg,
                HighlightColor::ElimBg,
            ),
            Highlight::new_candidate_hl(
                &CellCandidate::from_cell(other_wing, val),
                HighlightColor::NoteFg,
                HighlightColor::NoteBg,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rectangle_elimination() {
        let bd =
            "200709006190000002080002030670503040409000305350904060060300090800090053900407001";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![CellCandidate::from(0, 4, 1)];
        expected.sort();

        let rectangle_elimination = find_rectangle_elimination(&grid).unwrap();
        let to_place = rectangle_elimination.get_to_place().clone();
        let mut to_eliminate = rectangle_elimination.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!(Vec::<CellCandidate>::new(), to_place);
        assert_eq!(expected, to_eliminate);
    }
}
