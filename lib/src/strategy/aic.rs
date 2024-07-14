use std::collections::VecDeque;

use super::{
    highlight::{Highlight, HighlightColor},
    link::{make_link_map, LinkMap, LinkNode, LinkType},
    StrategyResult,
};
use crate::grid::{CellCandidate, Grid};

use LinkType::{StrongInCell, StrongInUnit, WeakInCell, WeakInUnit};

pub type AIC = Vec<LinkNode>;

pub enum DiscontinuousType {
    Weak(Vec<LinkNode>),
    Strong(LinkNode),
}

pub enum AICType {
    Continuous,
    Discontinuous(DiscontinuousType),
}

pub struct AICResult {
    aic: AIC,
    aic_type: AICType,
    to_eliminate: Vec<CellCandidate>,
}

impl AICResult {
    pub fn from(aic: &AIC, aic_type: AICType, to_eliminate: Vec<CellCandidate>) -> AICResult {
        AICResult {
            aic: aic.clone(),
            aic_type,
            to_eliminate,
        }
    }

    pub fn get_aic(&self) -> &AIC {
        &self.aic
    }

    pub fn get_aic_type(&self) -> &AICType {
        &self.aic_type
    }

    pub fn get_to_eliminate(&self) -> &Vec<CellCandidate> {
        &self.to_eliminate
    }

    pub fn make_highlights(&self, alternate_color: bool, make_lines: bool) -> Vec<Highlight> {
        let mut highlights = vec![];

        let aic = self.get_aic();

        for i in 0..aic.len() {
            let next_idx = if i < aic.len() - 1 { i + 1 } else { 0 };

            let current = &aic[i].get()[0];
            let next = &aic[next_idx].get()[0];

            let line_exists =
                !(matches!(self.aic_type, AICType::Discontinuous(_)) && i == aic.len() - 1);

            if make_lines && line_exists && !current.same_cell(next) {
                highlights.push(Highlight::new_line_hl(
                    current,
                    next,
                    HighlightColor::Red,
                    (i % 2) == 1,
                ));
            }

            if (i % 2) == 0 {
                highlights.push(Highlight::new_candidate_hl(
                    current,
                    if alternate_color {
                        HighlightColor::NoteSecondaryFg
                    } else {
                        HighlightColor::NoteFg
                    },
                    if alternate_color {
                        HighlightColor::NoteSecondaryBg
                    } else {
                        HighlightColor::NoteBg
                    },
                ));
            } else {
                highlights.push(Highlight::new_candidate_hl(
                    current,
                    HighlightColor::NoteFg,
                    HighlightColor::NoteBg,
                ));
            }
        }

        highlights
    }
}

pub fn find_general_aic(grid: &Grid) -> Option<StrategyResult> {
    let strong_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit]);
    let weak_link_map = make_link_map(grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

    if let Some(aic_result) = build_aics(&strong_link_map, &weak_link_map, 12) {
        let highlights = make_highlights(&aic_result);

        match aic_result.get_aic_type() {
            AICType::Continuous => {
                return Some(StrategyResult::from(
                    "Continuous AIC loop",
                    vec![],
                    aic_result.get_to_eliminate().clone(),
                    highlights,
                ));
            }

            AICType::Discontinuous(disc_type) => {
                let name = match disc_type {
                    DiscontinuousType::Weak(_) => "Discontinuous AIC Loop",
                    DiscontinuousType::Strong(_) => "Discontinuous AIC Loop (strong link)",
                };

                return Some(StrategyResult::from(
                    name,
                    vec![],
                    aic_result.get_to_eliminate().clone(),
                    highlights,
                ));
            }
        }
    }

    None
}

pub fn build_aics(
    strong_link_map: &LinkMap,
    weak_link_map: &LinkMap,
    max_length: usize,
) -> Option<AICResult> {
    for start_node in strong_link_map.keys() {
        let mut to_visit = VecDeque::from([AIC::from([start_node.clone()])]);

        while !to_visit.is_empty() {
            let current_path = to_visit.pop_front().unwrap();
            let current_node = current_path.last().unwrap();

            if current_path.len() > max_length {
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

                if search_strong_link && new_path.len() > 2 {
                    if let Some(found) = check_aic(&new_path, strong_link_map, weak_link_map) {
                        return Some(found);
                    }
                }

                to_visit.push_back(new_path);
            }
        }
    }

    None
}

fn check_aic(aic: &AIC, strong_link_map: &LinkMap, weak_link_map: &LinkMap) -> Option<AICResult> {
    if let Some(res) = check_continuous(aic, weak_link_map) {
        return Some(res);
    }

    if let Some(res) = check_discontinuous(aic, strong_link_map, weak_link_map) {
        return Some(res);
    }

    None
}

fn check_continuous(aic: &AIC, weak_link_map: &LinkMap) -> Option<AICResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if !weak_link_map.get(start).unwrap().contains(end) {
        return None;
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

        for link_node in linked_to_both {
            for cell_candidate in link_node.get().iter() {
                to_eliminate.push(cell_candidate.clone());
            }
        }
    }

    if to_eliminate.is_empty() {
        return None;
    }

    Some(AICResult::from(aic, AICType::Continuous, to_eliminate))
}

fn check_discontinuous(
    aic: &AIC,
    strong_link_map: &LinkMap,
    weak_link_map: &LinkMap,
) -> Option<AICResult> {
    let start = aic.first().unwrap();
    let end = aic.last().unwrap();

    if start.same_cell(end) {
        return None;
    }

    let linked_to_both = weak_link_map
        .get(start)
        .unwrap()
        .intersection(weak_link_map.get(end).unwrap());

    let mut to_eliminate = vec![];
    let mut discontinuities = vec![];
    let mut is_strong = false;

    for discontinuity in linked_to_both {
        if strong_link_map.get(start).unwrap().contains(discontinuity) {
            to_eliminate = vec![];

            for link_node in weak_link_map.get(start).unwrap().iter() {
                for cell_candidate in link_node.get().iter() {
                    to_eliminate.push(cell_candidate.clone());
                }
            }

            is_strong = true;
            discontinuities = vec![discontinuity.clone()];

            break;
        } else {
            for cell_candidate in discontinuity.get().iter() {
                to_eliminate.push(cell_candidate.clone());
            }

            discontinuities.push(discontinuity.clone());
        }
    }

    let disc_type = if is_strong {
        DiscontinuousType::Strong(discontinuities[0].clone())
    } else {
        DiscontinuousType::Weak(discontinuities)
    };

    if to_eliminate.is_empty() {
        None
    } else {
        Some(AICResult::from(
            aic,
            AICType::Discontinuous(disc_type),
            to_eliminate,
        ))
    }
}

fn make_highlights(aic_result: &AICResult) -> Vec<Highlight> {
    let mut highlights = aic_result.make_highlights(true, true);

    for cell_candidate in aic_result.get_to_eliminate().iter() {
        highlights.push(Highlight::new_candidate_hl(
            cell_candidate,
            HighlightColor::ElimFg,
            HighlightColor::ElimBg,
        ));
    }

    if let AICType::Discontinuous(disc_type) = aic_result.get_aic_type() {
        let start = &aic_result.get_aic().first().unwrap().get()[0];
        let end = &aic_result.get_aic().last().unwrap().get()[0];

        match disc_type {
            DiscontinuousType::Strong(link_node) => {
                let disc = &link_node.get()[0];

                if !disc.same_cell(end) {
                    // weak link from end to discontinuity
                    highlights.push(Highlight::new_line_hl(
                        end,
                        disc,
                        HighlightColor::Red,
                        true,
                    ));
                }

                if !disc.same_cell(start) {
                    // strong link from discontinuity to start
                    highlights.push(Highlight::new_line_hl(
                        disc,
                        start,
                        HighlightColor::Red,
                        false,
                    ));
                }
            }
            DiscontinuousType::Weak(link_nodes) => {
                // weak links from end -> discontinuity -> start
                for link_node in link_nodes.iter() {
                    let disc = &link_node.get()[0];

                    if !disc.same_cell(end) {
                        highlights.push(Highlight::new_line_hl(
                            end,
                            disc,
                            HighlightColor::Red,
                            true,
                        ));
                    }

                    if !disc.same_cell(start) {
                        highlights.push(Highlight::new_line_hl(
                            disc,
                            start,
                            HighlightColor::Red,
                            true,
                        ));
                    }
                }
            }
        };
    };

    highlights
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_check_aic(
        aic: &AIC,
        strong_link_map: &LinkMap,
        weak_link_map: &LinkMap,
    ) -> Option<AICResult> {
        if let Some(res) = check_aic(aic, strong_link_map, weak_link_map) {
            return Some(res);
        }

        None
    }

    fn all_in_bd(bd: &str) -> Vec<StrategyResult> {
        let grid = Grid::from_str(bd).unwrap();

        let strong_link_map = make_link_map(&grid, &[StrongInCell, StrongInUnit]);
        let weak_link_map =
            make_link_map(&grid, &[StrongInCell, StrongInUnit, WeakInCell, WeakInUnit]);

        let aics = build_aics(&strong_link_map, &weak_link_map, 12);

        vec![]

        // let mut lengths: Vec<usize> = aics.keys().cloned().collect();
        // lengths.sort();
        //
        // let mut results_all = vec![];
        //
        // for l in lengths.iter() {
        //     for aic in aics.get(l).unwrap().iter() {
        //         if let Some(results) = test_check_aic(aic, &strong_link_map, &weak_link_map) {
        //             for res in results.into_iter() {
        //                 println!("{:?}", res);
        //                 results_all.push(res);
        //             }
        //         }
        //     }
        // }
        //
        // results_all
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

        // panic!();
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

        // panic!();
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

        // panic!();
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

        // panic!();
    }
}
