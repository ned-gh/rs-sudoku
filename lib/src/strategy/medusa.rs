use itertools::Itertools;

use std::collections::{HashMap, HashSet};

use super::{
    link::{make_link_map, LinkMap, LinkType},
    StrategyResult,
};
use crate::grid::{get_minigrid_n_from_coords, CellCandidate, Grid};

type CellCoords = (u32, u32);
type ColorMap = HashMap<CellCandidate, Color>;
type CellColorMap = HashMap<CellCoords, HashMap<u32, Color>>;
type InverseColorMap = HashMap<Color, Vec<CellCandidate>>;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Color {
    ColorA,
    ColorB,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            ColorA => ColorB,
            ColorB => ColorA,
        }
    }
}

use Color::{ColorA, ColorB};
use LinkType::{StrongInCell, StrongInUnit};

pub fn find_medusa(grid: &Grid) -> Option<StrategyResult> {
    let strong_link_map = make_link_map(grid, &[StrongInUnit, StrongInCell]);
    let component_starts = get_component_starts(&strong_link_map);

    for start in component_starts.iter() {
        let (color_map, cell_color_map) = color_component(start, &strong_link_map);

        // rule 1
        if let Some(res) = twice_in_a_cell(&cell_color_map, &color_map) {
            return Some(res);
        }

        let inverse_color_map = get_inverse_color_map(&color_map);

        // rule 2
        if let Some(res) = twice_in_a_unit(&inverse_color_map) {
            return Some(res);
        }

        // rule 3
        if let Some(res) = two_colors_in_cell(grid, &cell_color_map) {
            return Some(res);
        }

        // rule 4
        if let Some(res) = two_colors_elsewhere(grid, &color_map) {
            return Some(res);
        }

        // rule 5
        if let Some(res) = two_colors_unit_cell(grid, &color_map, &inverse_color_map) {
            return Some(res);
        }

        // rule 6
        if let Some(res) = cell_emptied_by_color(grid, &color_map, &inverse_color_map) {
            return Some(res);
        }
    }

    None
}

fn get_component_starts(strong_link_map: &LinkMap) -> Vec<CellCandidate> {
    let mut component_starts = vec![];

    let mut visited = HashSet::new();

    while visited.len() < strong_link_map.len() {
        let mut start = None;
        let mut to_visit = vec![];

        for cell_candidate in strong_link_map.keys() {
            if !visited.contains(cell_candidate) {
                to_visit.push(cell_candidate.clone());
                break;
            }
        }

        while let Some(current) = to_visit.pop() {
            if !visited.contains(&current) {
                for next in strong_link_map.get(&current).unwrap() {
                    to_visit.push(next.clone());
                }

                if start.is_none() {
                    start = Some(current.clone());
                }

                visited.insert(current);
            }
        }

        if let Some(cell_candidate) = start {
            component_starts.push(cell_candidate);
        }
    }

    component_starts
}

fn color_component(start: &CellCandidate, strong_link_map: &LinkMap) -> (ColorMap, CellColorMap) {
    let mut color_map = ColorMap::new();
    let mut cell_color_map = CellColorMap::new();

    color_map.insert(start.clone(), ColorA);

    let (r, c, val) = start.as_tuple();
    cell_color_map.insert((r, c), HashMap::new());
    cell_color_map.get_mut(&(r, c)).unwrap().insert(val, ColorA);

    let mut visited = HashSet::new();
    let mut to_visit = vec![start.clone()];

    while let Some(current) = to_visit.pop() {
        if !visited.contains(&current) {
            let next_color = color_map.get(&current).unwrap().opposite();

            for next in strong_link_map.get(&current).unwrap().iter() {
                color_map.insert(next.clone(), next_color);

                let (nr, nc, nval) = next.as_tuple();

                cell_color_map.entry((nr, nc)).or_insert_with(HashMap::new);

                cell_color_map
                    .get_mut(&(nr, nc))
                    .unwrap()
                    .insert(nval, next_color);

                to_visit.push(next.clone());
            }

            visited.insert(current.clone());
        }
    }

    (color_map, cell_color_map)
}

fn get_inverse_color_map(color_map: &ColorMap) -> InverseColorMap {
    let mut inverse_color_map = InverseColorMap::from([(ColorA, vec![]), (ColorB, vec![])]);

    for (cell_candidate, color) in color_map.iter() {
        inverse_color_map
            .get_mut(color)
            .unwrap()
            .push(cell_candidate.clone());
    }

    inverse_color_map
}

fn twice_in_a_cell(cell_color_map: &CellColorMap, color_map: &ColorMap) -> Option<StrategyResult> {
    for val_colors in cell_color_map.values() {
        let mut color_count = [0, 0];

        for color in val_colors.values() {
            match color {
                ColorA => color_count[0] += 1,
                ColorB => color_count[1] += 1,
            }
        }

        for (i, count) in color_count.iter().enumerate() {
            if *count != 2 {
                continue;
            }

            let twice_color = if i == 0 { ColorA } else { ColorB };

            let mut to_place = vec![];

            for (cell_candidate, color) in color_map.iter() {
                if *color != twice_color {
                    to_place.push(cell_candidate.clone());
                }
            }

            return Some(StrategyResult::from(
                "Medusa - Twice in a Cell",
                to_place,
                vec![],
            ));
        }
    }

    None
}

fn twice_in_a_unit(inverse_color_map: &InverseColorMap) -> Option<StrategyResult> {
    for (color, cell_candidates) in inverse_color_map.iter() {
        let mut unit_counts = [[[0; 9]; 3]; 10];

        for cell_candidate in cell_candidates.iter() {
            let (r, c, val) = cell_candidate.as_tuple();
            let mg = get_minigrid_n_from_coords(r, c);

            for (i, &n) in [r, c, mg].iter().enumerate() {
                if unit_counts[val as usize][i][n as usize] == 0 {
                    unit_counts[val as usize][i][n as usize] += 1;
                    continue;
                }

                let to_place = inverse_color_map.get(&color.opposite()).unwrap().clone();

                return Some(StrategyResult::from(
                    "Medusa - Twice in a Unit",
                    to_place,
                    vec![],
                ));
            }
        }
    }

    None
}

fn two_colors_in_cell(grid: &Grid, cell_color_map: &CellColorMap) -> Option<StrategyResult> {
    let mut to_eliminate = vec![];

    for (&(r, c), val_colors) in cell_color_map.iter() {
        if val_colors.len() == 1 {
            continue;
        }

        for val in grid.get_candidates(r, c).iter() {
            if !val_colors.contains_key(&val) {
                to_eliminate.push(CellCandidate::from(r, c, val));
            }
        }
    }

    if to_eliminate.is_empty() {
        None
    } else {
        Some(StrategyResult::from(
            "Medusa - Two Colors in a Cell",
            vec![],
            to_eliminate,
        ))
    }
}

fn two_colors_elsewhere(grid: &Grid, color_map: &ColorMap) -> Option<StrategyResult> {
    let mut val_color_map = HashMap::new();

    for (cell_candidate, color) in color_map.iter() {
        let (r, c, val) = cell_candidate.as_tuple();

        val_color_map.entry(val).or_insert_with(HashMap::new);

        val_color_map
            .entry(val)
            .or_insert(HashMap::new())
            .entry(*color)
            .or_insert(vec![])
            .push((r, c));
    }

    let mut to_eliminate = HashSet::new();

    for (&val, cmap) in val_color_map.iter() {
        if cmap.len() < 2 {
            continue;
        }

        let color_a_cells = cmap.get(&ColorA).unwrap();
        let color_b_cells = cmap.get(&ColorB).unwrap();

        for (&(ar, ac), &(br, bc)) in color_a_cells.iter().cartesian_product(color_b_cells.iter()) {
            let sees_both = grid
                .get_cells_that_see_coords(ar, ac, false)
                .intersection(&grid.get_cells_that_see_coords(br, bc, false))
                .scan(val);

            for cell in sees_both.iter() {
                to_eliminate.insert(CellCandidate::from_cell(cell, val));
            }
        }
    }

    if to_eliminate.is_empty() {
        None
    } else {
        Some(StrategyResult::from(
            "Medusa - Two Colors Elsewhere",
            vec![],
            to_eliminate.into_iter().collect(),
        ))
    }
}

fn two_colors_unit_cell(
    grid: &Grid,
    color_map: &ColorMap,
    inverse_color_map: &InverseColorMap,
) -> Option<StrategyResult> {
    let mut to_eliminate = vec![];

    for (cell_candidate, color) in color_map.iter() {
        let (r, c, val) = cell_candidate.as_tuple();

        let opposite_color = color.opposite();

        for other_val in grid.get_candidates(r, c).iter() {
            if other_val == val {
                continue;
            }

            let other_cell_candidate = CellCandidate::from(r, c, other_val);

            for opposite_cell_candidate in inverse_color_map.get(&opposite_color).unwrap().iter() {
                if other_cell_candidate.same_cell(opposite_cell_candidate) {
                    continue;
                }

                if other_cell_candidate.can_see(opposite_cell_candidate, true) {
                    to_eliminate.push(other_cell_candidate);
                    break;
                }
            }
        }
    }

    if to_eliminate.is_empty() {
        None
    } else {
        Some(StrategyResult::from(
            "Medusa - Two Colors Unit + Cell",
            vec![],
            to_eliminate,
        ))
    }
}

fn cell_emptied_by_color(
    grid: &Grid,
    color_map: &ColorMap,
    inverse_color_map: &InverseColorMap,
) -> Option<StrategyResult> {
    let mut colored_cells = HashSet::new();

    for cell_candidate in color_map.keys() {
        let (r, c, _) = cell_candidate.as_tuple();
        colored_cells.insert((r, c));
    }

    for cell in grid.iter() {
        let (r, c) = (cell.get_row(), cell.get_col());

        if colored_cells.contains(&(r, c)) {
            continue;
        }

        for (color, cell_candidates) in inverse_color_map.iter() {
            let mut all_see_color = true;

            for val in cell.get_candidates().iter() {
                let target = CellCandidate::from(r, c, val);

                let mut val_sees_color = false;

                for colored_candidate in cell_candidates.iter() {
                    if target.can_see(colored_candidate, true) {
                        val_sees_color = true;
                        break;
                    }
                }

                if !val_sees_color {
                    all_see_color = false;
                    break;
                }
            }

            if all_see_color {
                let opposite_color = color.opposite();

                let to_place = inverse_color_map.get(&opposite_color).unwrap().clone();

                return Some(StrategyResult::from(
                    "Medusa - Cell Emptied by Color",
                    to_place,
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

    fn setup(grid: &Grid) -> (LinkMap, Vec<CellCandidate>) {
        let strong_link_map = make_link_map(grid, &[StrongInUnit, StrongInCell]);
        let component_starts = get_component_starts(&strong_link_map);

        (strong_link_map, component_starts)
    }

    #[test]
    fn test_twice_in_a_cell() {
        let bd =
            "093824560085600002206075008321769845000258300578040296850016723007082650002507180";
        let grid = Grid::from_str(bd).unwrap();

        let (strong_link_map, component_starts) = setup(&grid);

        let mut expected = vec![
            CellCandidate::from(7, 3, 3),
            CellCandidate::from(2, 1, 1),
            CellCandidate::from(2, 3, 9),
            CellCandidate::from(7, 0, 1),
            CellCandidate::from(1, 0, 4),
            CellCandidate::from(2, 6, 4),
            CellCandidate::from(1, 6, 9),
            CellCandidate::from(8, 1, 3),
            CellCandidate::from(1, 4, 3),
            CellCandidate::from(8, 4, 9),
        ];
        expected.sort();

        let mut results = vec![];

        for start in component_starts.iter() {
            let (color_map, cell_color_map) = color_component(start, &strong_link_map);

            if let Some(res) = twice_in_a_cell(&cell_color_map, &color_map) {
                let mut to_place = res.get_to_place().clone();
                to_place.sort();

                results.push(to_place);
            }
        }

        assert!(results.contains(&expected));
    }

    #[test]
    fn test_twice_in_a_unit() {
        let bd =
            "300052000250300010004607523093200805570000030408035060005408300030506084840023056";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid correctly
        grid.clear_candidate(&CellCandidate::from(0, 6, 7));
        grid.clear_candidate(&CellCandidate::from(0, 6, 9));
        grid.clear_candidate(&CellCandidate::from(1, 6, 4));
        grid.clear_candidate(&CellCandidate::from(1, 6, 9));
        grid.clear_candidate(&CellCandidate::from(3, 4, 1));
        grid.clear_candidate(&CellCandidate::from(4, 3, 1));
        grid.clear_candidate(&CellCandidate::from(4, 4, 1));
        grid.clear_candidate(&CellCandidate::from(4, 4, 4));
        grid.clear_candidate(&CellCandidate::from(4, 8, 2));
        grid.clear_candidate(&CellCandidate::from(5, 3, 1));
        grid.clear_candidate(&CellCandidate::from(5, 6, 2));
        grid.clear_candidate(&CellCandidate::from(5, 8, 9));
        grid.clear_candidate(&CellCandidate::from(7, 2, 7));
        grid.clear_candidate(&CellCandidate::from(8, 2, 7));

        let (strong_link_map, component_starts) = setup(&grid);

        let expected = vec![
            CellCandidate::from(0, 6, 4),
            CellCandidate::from(1, 6, 6),
            CellCandidate::from(3, 4, 7),
            CellCandidate::from(3, 7, 4),
            CellCandidate::from(4, 5, 4),
            CellCandidate::from(5, 3, 9),
            CellCandidate::from(8, 3, 7),
        ];

        let mut results = vec![];

        for start in component_starts.iter() {
            let (color_map, _) = color_component(start, &strong_link_map);
            let inverse_color_map = get_inverse_color_map(&color_map);

            if let Some(res) = twice_in_a_unit(&inverse_color_map) {
                let mut to_place = res.get_to_place().clone();
                to_place.sort();

                results.push(to_place);
            }
        }

        assert!(results.contains(&expected));
    }

    #[test]
    fn test_two_colors_in_cell() {
        let bd =
            "290000830000020970000109402845761293600000547009045008903407000060030709050000384";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid correctly
        grid.clear_candidate(&CellCandidate::from(1, 0, 3));
        grid.clear_candidate(&CellCandidate::from(1, 1, 3));
        grid.clear_candidate(&CellCandidate::from(5, 0, 1));
        grid.clear_candidate(&CellCandidate::from(5, 1, 1));

        let (strong_link_map, component_starts) = setup(&grid);

        let expected = vec![CellCandidate::from(2, 1, 8)];

        let mut results = vec![];

        for start in component_starts.iter() {
            let (_, cell_color_map) = color_component(start, &strong_link_map);

            if let Some(res) = two_colors_in_cell(&grid, &cell_color_map) {
                results.push(res.get_to_eliminate().clone());
            }
        }

        assert!(results.contains(&expected));
    }

    #[test]
    fn test_two_colors_elsewhere() {
        let bd =
            "100056003043090000800043002030560210950421037021030000317980005000310970000670301";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid correctly
        grid.clear_candidate(&CellCandidate::from(1, 0, 7));
        grid.clear_candidate(&CellCandidate::from(1, 5, 2));
        grid.clear_candidate(&CellCandidate::from(2, 6, 6));
        grid.clear_candidate(&CellCandidate::from(3, 8, 8));
        grid.clear_candidate(&CellCandidate::from(5, 7, 9));
        grid.clear_candidate(&CellCandidate::from(5, 8, 8));
        grid.clear_candidate(&CellCandidate::from(7, 2, 6));
        grid.clear_candidate(&CellCandidate::from(7, 2, 8));
        grid.clear_candidate(&CellCandidate::from(7, 8, 6));
        grid.clear_candidate(&CellCandidate::from(8, 2, 8));

        let (strong_link_map, component_starts) = setup(&grid);

        let expected = vec![CellCandidate::from(1, 0, 6), CellCandidate::from(2, 7, 6)];

        let mut results = vec![];

        for start in component_starts.iter() {
            let (color_map, _) = color_component(start, &strong_link_map);

            if let Some(res) = two_colors_elsewhere(&grid, &color_map) {
                let mut to_eliminate = res.get_to_eliminate().clone();
                to_eliminate.sort();

                results.push(to_eliminate);
            }
        }

        assert!(results.contains(&expected));
    }

    #[test]
    fn test_two_colors_unit_cell() {
        let bd =
            "923407015876050924500200030769020140432000059185004260098042071207030486000708092";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid correctly
        grid.clear_candidate(&CellCandidate::from(2, 4, 1));
        grid.clear_candidate(&CellCandidate::from(2, 5, 1));
        grid.clear_candidate(&CellCandidate::from(4, 4, 8));

        let (strong_link_map, component_starts) = setup(&grid);

        let expected = vec![
            CellCandidate::from(2, 4, 8),
            CellCandidate::from(2, 6, 6),
            CellCandidate::from(4, 3, 6),
            CellCandidate::from(4, 4, 1),
        ];

        let mut results = vec![];

        for start in component_starts.iter() {
            let (color_map, _) = color_component(start, &strong_link_map);
            let inverse_color_map = get_inverse_color_map(&color_map);

            if let Some(res) = two_colors_unit_cell(&grid, &color_map, &inverse_color_map) {
                let mut to_eliminate = res.get_to_eliminate().clone();
                to_eliminate.sort();

                results.push(to_eliminate);
            }
        }

        assert!(results.contains(&expected));
    }

    #[test]
    fn test_cell_emptied_by_color() {
        let bd =
            "986721345304956007007030960073065009690017003100390276000679030069143700731582694";
        let mut grid = Grid::from_str(bd).unwrap();

        // set up grid correctly
        grid.clear_candidate(&CellCandidate::from(2, 8, 8));
        grid.clear_candidate(&CellCandidate::from(4, 2, 5));
        grid.clear_candidate(&CellCandidate::from(6, 6, 8));
        grid.clear_candidate(&CellCandidate::from(7, 7, 8));

        let (strong_link_map, component_starts) = setup(&grid);

        let expected = vec![
            CellCandidate::from(1, 1, 1),
            CellCandidate::from(1, 7, 2),
            CellCandidate::from(2, 8, 1),
            CellCandidate::from(4, 6, 5),
            CellCandidate::from(4, 7, 8),
            CellCandidate::from(6, 6, 1),
            CellCandidate::from(7, 7, 5),
        ];

        let mut results = vec![];

        for start in component_starts.iter() {
            let (color_map, _) = color_component(start, &strong_link_map);
            let inverse_color_map = get_inverse_color_map(&color_map);

            if let Some(res) = cell_emptied_by_color(&grid, &color_map, &inverse_color_map) {
                let mut to_place = res.get_to_place().clone();
                to_place.sort();

                results.push(to_place);
            }
        }

        assert!(results.contains(&expected));
    }
}
