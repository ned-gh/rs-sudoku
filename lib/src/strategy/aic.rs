use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    link::{make_link_map, LinkMap, LinkType},
    StrategyResult,
};
use crate::grid::{CellCandidate, Grid};

type AIC = Vec<CellCandidate>;
type AICMap = HashMap<usize, HashSet<AIC>>;

use LinkType::{StrongInCell, StrongInUnit, WeakInCell, WeakInUnit};

pub fn find_aic(grid: &Grid) -> Option<StrategyResult> {
    let strong_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit]);
    let weak_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

    let aics = build_aics(&strong_link_map, &weak_link_map);

    let mut lengths: Vec<usize> = aics.keys().cloned().collect();
    lengths.sort();

    for l in lengths.iter() {
        for aic in aics.get(l).unwrap().iter() {
            if let Some(res) = check_type1(grid, aic) {
                return Some(res);
            }

            if let Some(res) = check_type2(grid, aic) {
                return Some(res);
            }

            if let Some(res) = check_continuous(grid, aic) {
                return Some(res);
            }

            if let Some(res) = check_discontinuous(grid, aic, &strong_link_map) {
                return Some(res);
            }
        }
    }

    None
}

fn build_aics(strong_link_map: &LinkMap, weak_link_map: &LinkMap) -> AICMap {
    let mut aics: AICMap = HashMap::new();

    for start_node in strong_link_map.keys() {
        let mut to_visit = VecDeque::from([AIC::from([start_node.clone()])]);

        while !to_visit.is_empty() {
            let current_path = to_visit.pop_front().unwrap();
            let current_node = current_path.last().unwrap();

            if current_path.len() > 12 {
                continue;
            }

            let search_strong_link = (current_path.len() % 2) == 1;

            let search_map = if search_strong_link {
                strong_link_map
            } else {
                weak_link_map
            };

            if !search_map.contains_key(current_node) {
                continue;
            }

            for node in search_map.get(current_node).unwrap().iter() {
                if current_path.contains(node) {
                    continue;
                }

                let mut new_path = current_path.clone();
                new_path.push(node.clone());

                let new_len = new_path.len();

                if search_strong_link && new_len > 2 {
                    let aic_set = aics.entry(new_len).or_insert_with(HashSet::new);
                    aic_set.insert(new_path.clone());
                }

                to_visit.push_back(new_path);
            }
        }
    }

    aics
}

fn check_type1(grid: &Grid, aic: &AIC) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if start.get_val() != end.get_val() {
        return None;
    }

    let (start_r, start_c, val) = start.as_tuple();
    let (end_r, end_c, _) = end.as_tuple();

    let cells = grid
        .get_cells_that_see_coords(start_r, start_c, false)
        .intersection(&grid.get_cells_that_see_coords(end_r, end_c, false))
        .scan(val);

    if cells.is_empty() {
        return None;
    }

    Some(StrategyResult::from(
        &format!("AIC Type 1: {}", aic_to_string(aic)),
        vec![],
        cells
            .iter()
            .map(|cell| CellCandidate::from_cell(cell, val))
            .collect(),
    ))
}

fn check_type2(grid: &Grid, aic: &AIC) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, start_val) = start.as_tuple();
    let (end_r, end_c, end_val) = end.as_tuple();

    if (start_r, start_c) == (end_r, end_c) || start_val == end_val || !start.can_see(end, false) {
        return None;
    }

    let mut cell_candidates = vec![];

    if grid.get_candidates(end_r, end_c).contains(start_val) {
        cell_candidates.push(CellCandidate::from(end_r, end_c, start_val));
    }

    if grid.get_candidates(start_r, start_c).contains(end_val) {
        cell_candidates.push(CellCandidate::from(start_r, start_c, end_val));
    }

    if cell_candidates.is_empty() {
        return None;
    }

    Some(StrategyResult::from(
        &format!("AIC Type 2: {}", aic_to_string(aic)),
        vec![],
        cell_candidates
    ))
}

fn check_continuous(grid: &Grid, aic: &AIC) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, _) = start.as_tuple();
    let (end_r, end_c, _) = end.as_tuple();

    if (start_r, start_c) != (end_r, end_c) {
        return None;
    }

    let mut cell_candidates = vec![];

    for i in 0..aic.len() {
        if (i % 2) == 0 {
            continue;
        }

        let next = if i < aic.len() - 1 { i + 1 } else { 0 };

        let (ir, ic, iv) = aic[i].as_tuple();
        let (nr, nc, nv) = aic[next].as_tuple();

        if (ir, ic) == (nr, nc) {
            // elim within cell
            for val in grid.get_candidates(ir, ic).iter() {
                if val != iv && val != nv {
                    cell_candidates.push(CellCandidate::from(ir, ic, val));
                }
            }
        } else if iv == nv {
            // elim within unit
            let sees_both = grid
                .get_cells_that_see_coords(ir, ic, false)
                .intersection(&grid.get_cells_that_see_coords(nr, nc, false));

            for cell in sees_both.iter() {
                if !cell.get_candidates().contains(iv) {
                    continue;
                }
                cell_candidates.push(CellCandidate::from_cell(cell, iv));
            }
        }
    }

    if cell_candidates.is_empty() {
        return None;
    }

    Some(StrategyResult::from(
        &format!("Continuous AIC Loop: {}", aic_to_string(aic)),
        vec![],
        cell_candidates
    ))
}

fn check_discontinuous(
    grid: &Grid,
    aic: &AIC,
    strong_link_map: &LinkMap,
) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, start_val) = start.as_tuple();
    let (end_r, end_c, end_val) = end.as_tuple();

    if (start_r, start_c) == (end_r, end_c) || start_val == end_val || !start.can_see(end, false) {
        return None;
    }

    if !grid.get_candidates(end_r, end_c).contains(start_val) {
        return None;
    }

    let discontinuity = CellCandidate::from(end_r, end_c, start_val);

    if strong_link_map.get(start).unwrap().contains(&discontinuity) {
        Some(StrategyResult::from(
            &format!("Discontinuous AIC Loop: {}", aic_to_string(aic)),
            vec![start.clone()],
            vec![]
            ))
    } else {
        Some(StrategyResult::from(
            &format!("Discontinuous AIC Loop: {}", aic_to_string(aic)),
            vec![],
            vec![discontinuity]
        ))
    }
}

fn aic_to_string(aic: &AIC) -> String {
    let mut s = String::new();

    for (i, cell) in aic.iter().enumerate() {
        s.push_str(&format!(
            "({})r{}c{}",
            cell.get_val(),
            cell.get_row(),
            cell.get_col()
        ));

        if i < aic.len() - 1 {
            if i % 2 == 0 {
                s.push('=');
            } else {
                s.push('-');
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_aic(grid: &Grid, aic: &AIC, strong_link_map: &LinkMap) -> Option<Vec<StrategyResult>> {
        let mut results = vec![];

        if let Some(res) = check_type1(grid, aic) {
            results.push(res);
        }

        if let Some(res) = check_type2(grid, aic) {
            results.push(res);
        }

        if let Some(res) = check_continuous(grid, aic) {
            results.push(res);
        }

        if let Some(res) = check_discontinuous(grid, aic, strong_link_map) {
            results.push(res);
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    #[test]
    fn test_aic() {
        // has discontinuous (weak link)
        // let bd =
        //     "000080200005000040020005000962837000003214697174500832001000000697348521248751369";

        // has discontinuous (strong link)
        // let bd =
        //     "307465100215798436400200000000680043004020001003040200001000007000002000530870910";

        // has continuous
        // let bd =
        //     "040060102027500496000004308410007985000050201000000607004000013061900024030001069";

        // hard
        let bd =
            "004005000010900340080002009705080020000203000090050801300500090076009010000300700";
        let grid = Grid::from_str(bd).unwrap();

        let strong_link_map = make_link_map(&grid, &[StrongInCell, StrongInUnit]);
        let weak_link_map =
            make_link_map(&grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

        let aics = build_aics(&strong_link_map, &weak_link_map);

        let mut lengths: Vec<usize> = aics.keys().cloned().collect();
        lengths.sort();

        for l in lengths.iter() {
            for aic in aics.get(l).unwrap().iter() {
                if let Some(results) = check_aic(&grid, aic, &strong_link_map) {
                    for res in results.iter() {
                        println!("{:?}", res);
                    }
                }
            }
        }
    }
}
