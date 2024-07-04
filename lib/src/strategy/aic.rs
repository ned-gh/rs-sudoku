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
            if let Some(res) = check_aic(aic, &strong_link_map, &weak_link_map) {
                return Some(res);
            }
        }
    }

    None
}

fn check_aic(aic: &AIC, strong_link_map: &LinkMap, weak_link_map: &LinkMap) -> Option<StrategyResult> {
    if let Some(res) = check_continuous(aic, &weak_link_map) {
        return Some(res);
    }

    if let Some(res) = check_discontinuous(aic, &strong_link_map, &weak_link_map) {
        return Some(res);
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

fn check_continuous(aic: &AIC, weak_link_map: &LinkMap) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if !weak_link_map.get(start).unwrap().contains(end) {
        return None
    }

    let mut to_eliminate = vec![];

    for i in 0..aic.len() {
        if (i % 2) == 0 {
            continue;
        }

        let next_idx = if i < aic.len() - 1 { i + 1 } else { 0 };

        let current = &aic[i];
        let next = &aic[next_idx];

        let linked_to_both = weak_link_map
            .get(current)
            .unwrap()
            .intersection(weak_link_map.get(next).unwrap());

        for cell_candidate in linked_to_both {
            to_eliminate.push(cell_candidate.clone());
        }
    }

    if to_eliminate.is_empty() {
        return None;
    }

    Some(StrategyResult::from(
        &format!("Continuous AIC Loop: {}", aic_to_string(aic)),
        vec![],
        to_eliminate,
    ))
}

fn check_discontinuous(
    aic: &AIC,
    strong_link_map: &LinkMap,
    weak_link_map: &LinkMap,
) -> Option<StrategyResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    let (start_r, start_c, _) = start.as_tuple();
    let (end_r, end_c, _) = end.as_tuple();

    if (start_r, start_c) == (end_r, end_c) {
        return None;
    }

    let linked_to_both = weak_link_map
        .get(start)
        .unwrap()
        .intersection(weak_link_map.get(end).unwrap());

    let mut to_eliminate = vec![];

    for discontinuity in linked_to_both {
        if strong_link_map.get(start).unwrap().contains(discontinuity) {
            return Some(StrategyResult::from(
                &format!("Discontinuous Nice Loop: {}", aic_to_string(aic)),
                vec![start.clone()],
                vec![],
            ));
        } else {
            to_eliminate.push(discontinuity.clone());
        }
    }

    if to_eliminate.is_empty() {
        None
    } else {
        let name = if to_eliminate.len() == 1 {
            format!("Discontinuous Nice Loop: {}", aic_to_string(aic))
        } else if to_eliminate.len() == 2 {
            let mut in_start_or_end = true;
            for cell_candidate in to_eliminate.iter() {
                in_start_or_end = cell_candidate.same_cell(start) || cell_candidate.same_cell(end);
            }

            if in_start_or_end {
                format!("AIC Type 2: {}", aic_to_string(aic))
            } else {
                format!("AIC Type 1: {}", aic_to_string(aic))
            }
        } else {
            format!("AIC Type 1: {}", aic_to_string(aic))
        };

        Some(StrategyResult::from(&name, vec![], to_eliminate))
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

    fn test_check_aic(
        aic: &AIC,
        strong_link_map: &LinkMap,
        weak_link_map: &LinkMap,
    ) -> Option<Vec<StrategyResult>> {
        let mut results = vec![];

        if let Some(res) = check_aic(aic, strong_link_map, weak_link_map) {
            results.push(res);
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    fn all_in_bd(bd: &str) -> Vec<StrategyResult> {
        let grid = Grid::from_str(bd).unwrap();

        let strong_link_map = make_link_map(&grid, &[StrongInCell, StrongInUnit]);
        let weak_link_map =
            make_link_map(&grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

        let aics = build_aics(&strong_link_map, &weak_link_map);

        let mut lengths: Vec<usize> = aics.keys().cloned().collect();
        lengths.sort();

        let mut results_all = vec![];

        for l in lengths.iter() {
            for aic in aics.get(l).unwrap().iter() {
                if let Some(results) = test_check_aic(aic, &strong_link_map, &weak_link_map) {
                    for res in results.into_iter() {
                        println!("{:?}", res);
                        results_all.push(res);
                    }
                }
            }
        }

        results_all
    }

    #[test]
    fn test_aic_discontinuous_weak() {
        // has discontinuous (weak link)
        let bd =
            "000080200005000040020005000962837000003214697174500832001000000697348521248751369";

        let results = all_in_bd(bd);

        let expected_eliminate = vec![CellCandidate::from(0, 7, 7)];

        for res in results.iter() {
            if *res.get_to_eliminate() == expected_eliminate {
                return;
            }
        }

        panic!();
    }

    #[test]
    fn test_aic_discontinuous_strong() {
        // has discontinuous (strong link)
        let bd =
            "307465100215798436400200000000680043004020001003040200001000007000002000530870910";

        let results = all_in_bd(bd);

        let expected_place = vec![CellCandidate::from(7, 1, 4)];

        for res in results.iter() {
            if *res.get_to_place() == expected_place {
                return;
            }
        }

        panic!();
    }

    #[test]
    fn test_aic_continuous() {
        // has continuous
        let bd =
            "040060102027500496000004308410007985000050201000000607004000013061900024030001069";

        let results = all_in_bd(bd);

        let expected_eliminate = vec![
            CellCandidate::from(1, 4, 3),
            CellCandidate::from(3, 3, 2),
            CellCandidate::from(5, 3, 2),
            CellCandidate::from(5, 4, 2),
            CellCandidate::from(5, 4, 3),
            CellCandidate::from(6, 5, 6),
            CellCandidate::from(6, 5, 8),
            CellCandidate::from(7, 5, 8),
        ];

        for res in results.iter() {
            let mut to_eliminate = res.get_to_eliminate().clone();
            to_eliminate.sort();

            if to_eliminate == expected_eliminate {
                return;
            }
        }

        panic!();
    }

    #[test]
    fn test_aic_type_2() {
        // has type 2
        let bd =
            "513000829678219543429008167390105096050000030006000950060003018030081690001602375";

        let results = all_in_bd(bd);

        let expected_eliminate = vec![CellCandidate::from(5, 1, 8), CellCandidate::from(5, 3, 4)];

        for res in results.iter() {
            let mut to_eliminate = res.get_to_eliminate().clone();
            to_eliminate.sort();

            if to_eliminate == expected_eliminate {
                return;
            }
        }

        panic!();
    }
}
