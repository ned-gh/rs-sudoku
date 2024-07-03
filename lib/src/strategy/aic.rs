use std::collections::{HashMap, HashSet, VecDeque};

use super::link::{make_link_map, LinkType};
use crate::grid::{Cell, CellCandidate, Grid, Region, Unit, UnitType};

type AIC = Vec<CellCandidate>;
type AICMap = HashMap<usize, HashSet<AIC>>;

use LinkType::{StrongInCell, StrongInUnit, WeakInCell, WeakInUnit};

fn find_aic(grid: &Grid) -> AICMap {
    let strong_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit]);
    let weak_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

    let mut aics: AICMap = HashMap::new();

    for start_node in strong_link_map.keys() {
        let mut to_visit = VecDeque::from([AIC::from([start_node.clone()])]);

        while !to_visit.is_empty() {
            let current_path = to_visit.pop_front().unwrap();
            let current_node = current_path.last().unwrap();

            let search_strong_link = (current_path.len() % 2) == 1;

            let search_map = if search_strong_link {
                &strong_link_map
            } else {
                &weak_link_map
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

fn check_aic(grid: &Grid, aic: &AIC) {
    check_type1(grid, aic);

    check_type2(grid, aic);

    check_continous(grid, aic);

    check_discontinuous(grid, aic);
}

fn check_type1(grid: &Grid, aic: &AIC) {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if start.get_val() != end.get_val() {
        return;
    }

    let (start_r, start_c, val) = start.as_tuple();
    let (end_r, end_c, _) = end.as_tuple();

    let cells = grid
        .get_cells_that_see_coords(start_r, start_c, false)
        .intersection(&grid.get_cells_that_see_coords(end_r, end_c, false))
        .scan(val);

    if cells.is_empty() {
        return;
    }

    println!("-----");
    println!("Type 1 AIC (length {}): {}", aic.len(), aic_to_string(aic));
    println!("-> elim:");

    for cell in cells.iter() {
        println!("  {:?}", CellCandidate::from_cell(cell, val));
    }
}

fn check_type2(grid: &Grid, aic: &AIC) {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if start.get_val() == end.get_val() {
        return;
    }

    if !start.can_see(end, false) {
        return;
    }

    let (start_r, start_c, start_val) = start.as_tuple();
    let (end_r, end_c, end_val) = end.as_tuple();

    let mut cells = vec![];

    if grid.get_candidates(end_r, end_c).contains(start_val) {
        cells.push(CellCandidate::from(end_r, end_c, start_val));
    }

    if grid.get_candidates(start_r, start_c).contains(end_val) {
        cells.push(CellCandidate::from(start_r, start_c, end_val));
    }

    if cells.is_empty() {
        return;
    }

    println!("-----");
    println!("Type 2 AIC (length {}): {}", aic.len(), aic_to_string(aic));
    println!("-> elim:");

    for cell in cells.iter() {
        println!("  {:?}", cell);
    }
}

fn check_continous(grid: &Grid, aic: &AIC) {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, _) = start.as_tuple();
    let (end_r, end_c, _) = end.as_tuple();

    if start_r != end_r || start_c != end_c {
        return;
    }

    let mut cells = vec![];

    for i in 0..aic.len() {
        if (i % 2) == 0 {
            continue;
        }

        let next = if i < aic.len() - 1 { i + 1 } else { 0 };

        let (ir, ic, iv) = aic[i].as_tuple();
        let (nr, nc, nv) = aic[next].as_tuple();

        if ir == nr && ic == nc {
            for val in grid.get_candidates(ir, ic).iter() {
                if val != iv && val != nv {
                    cells.push(CellCandidate::from(ir, ic, val));
                }
            }
        } else if iv == nv {
            let sees_both = grid
                .get_cells_that_see_coords(ir, ic, false)
                .intersection(&grid.get_cells_that_see_coords(nr, nc, false));

            for _cell in sees_both.iter() {
                if !_cell.get_candidates().contains(iv) {
                    continue;
                }
                cells.push(CellCandidate::from_cell(_cell, iv));
            }
        }
    }

    if cells.is_empty() {
        return;
    }

    println!("-----");
    println!(
        "Continuous AIC loop (length {}): {}",
        aic.len(),
        aic_to_string(aic)
    );
    println!("-> elim: ({} total)", cells.len());

    for cell in cells.iter() {
        println!("  {:?}", cell);
    }
}

fn check_discontinuous(grid: &Grid, aic: &AIC) {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, start_val) = start.as_tuple();
    let (end_r, end_c, end_val) = end.as_tuple();

    if (start_r, start_c) == (end_r, end_c) || start_val == end_val || !start.can_see(end, false) {
        return;
    }

    if !grid.get_candidates(end_r, end_c).contains(start_val) {
        return;
    }

    let reg = Region::from(&[
        Cell::from(start_r, start_c, grid.get_candidates(start_r, start_c)),
        Cell::from(end_r, end_c, grid.get_candidates(end_r, end_c)),
    ]);

    let cells;

    if let Some(unit) = reg.all_in_unit() {
        match unit {
            Unit::Row(n) => cells = grid.get_unit(&UnitType::Row, n).scan(start_val),
            Unit::Col(n) => cells = grid.get_unit(&UnitType::Col, n).scan(start_val),
            Unit::MiniGrid(n) => cells = grid.get_unit(&UnitType::MiniGrid, n).scan(start_val),
        }
    } else {
        return;
    }

    if cells.len() == 2 {
        println!("-----");
        println!(
            "Discontinuous AIC loop (length {}): {}",
            aic.len(),
            aic_to_string(aic)
        );
        println!(
            "-> elim: ({} total)",
            grid.get_candidates(start_r, start_c).len() - 1
        );

        for val in grid.get_candidates(start_r, start_c).iter() {
            if val == start_val {
                continue;
            }

            println!("  {:?}", CellCandidate::from(start_r, start_c, val));
        }

        println!("-> place:");
        println!("  {:?}", start);
    } else if cells.len() > 2 {
        println!("-----");
        println!(
            "Discontinuous AIC loop (length {}): {}",
            aic.len(),
            aic_to_string(aic)
        );
        println!("-> elim: ({} total)", 1);
        println!("  {:?}", CellCandidate::from(end_r, end_c, start_val));
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

    #[test]
    fn test_aic() {
        // has discontinuous (weak link)
        // let bd =
        //     "000080200005000040020005000962837000003214697174500832001000000697348521248751369";

        // has discontinuous (strong link)
        // let bd =
        //     "307465100215798436400200000000680043004020001003040200001000007000002000530870910";

        // has continuous
        let bd =
            "040060102027500496000004308410007985000050201000000607004000013061900024030001069";

        let grid = Grid::from_str(bd).unwrap();

        let aics = find_aic(&grid);

        let mut lengths: Vec<usize> = aics.keys().cloned().collect();
        lengths.sort();

        for l in lengths.iter() {
            for aic in aics.get(l).unwrap().iter() {
                check_aic(&grid, aic);
            }
        }
    }
}
