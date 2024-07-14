use std::collections::{HashMap, HashSet};

use crate::grid::{CellCandidate, Grid, UnitType};

use UnitType::{Col, MiniGrid, Row};

pub enum LinkType {
    StrongInCell,
    StrongInUnit,
    WeakInCell,
    WeakInUnit,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct LinkNode {
    cell_candidates: Vec<CellCandidate>,
}

impl LinkNode {
    fn from(cell_candidate: CellCandidate) -> LinkNode {
        LinkNode {
            cell_candidates: vec![cell_candidate],
        }
    }

    fn from_multi(cell_candidates: &[CellCandidate]) -> LinkNode {
        LinkNode {
            cell_candidates: cell_candidates.to_vec(),
        }
    }

    pub fn get(&self) -> &Vec<CellCandidate> {
        &self.cell_candidates
    }

    pub fn same_cell(&self, other: &LinkNode) -> bool {
        if self.cell_candidates.len() > 1 || other.cell_candidates.len() > 1 {
            return false;
        }

        let (self_r, self_c, _) = self.cell_candidates[0].as_tuple();
        let (other_r, other_c, _) = other.cell_candidates[0].as_tuple();

        self_r == other_r && self_c == other_c
    }

    pub fn get_singleton(&self) -> &CellCandidate {
        &self.cell_candidates[0]
    }
}

pub type LinkMap = HashMap<LinkNode, HashSet<LinkNode>>;

use LinkType::*;

pub fn make_link_map(grid: &Grid, link_types: &[LinkType]) -> LinkMap {
    let mut map = LinkMap::new();

    for link_type in link_types {
        merge(&mut map, make_from_link(grid, link_type));
    }

    map
}

fn merge(map: &mut LinkMap, other: LinkMap) {
    for (k, v) in other.into_iter() {
        if !map.contains_key(&k) {
            map.insert(k.clone(), HashSet::new());
        }

        map.get_mut(&k).unwrap().extend(v);
    }
}

fn make_from_link(grid: &Grid, link_type: &LinkType) -> LinkMap {
    match link_type {
        StrongInCell => make_in_cells(grid, true),
        StrongInUnit => make_in_units(grid, true),
        WeakInCell => make_in_cells(grid, false),
        WeakInUnit => make_in_units(grid, false),
    }
}

fn make_in_cells(grid: &Grid, only_strong_links: bool) -> LinkMap {
    let mut map = LinkMap::new();

    let cells = if only_strong_links {
        grid.get_nvalue_cells(2)
    } else {
        grid.as_region()
    };

    for cell in cells.iter() {
        let candidates = cell.get_candidates();

        for a in candidates.iter() {
            for b in candidates.iter() {
                if a == b {
                    continue;
                }

                let node_a = LinkNode::from(CellCandidate::from_cell(cell, a));
                let node_b = LinkNode::from(CellCandidate::from_cell(cell, b));

                map.entry(node_a).or_default().insert(node_b);
            }
        }
    }

    map
}

fn make_in_units(grid: &Grid, only_strong_links: bool) -> LinkMap {
    let mut map = LinkMap::new();

    for unit_type in &[Row, Col, MiniGrid] {
        for k in 0..9 {
            let unit = grid.get_unit(unit_type, k);

            for val in 1..10 {
                let cells = unit.scan(val);

                if (only_strong_links && cells.len() != 2) || (cells.len() < 2) {
                    continue;
                }

                for a in cells.iter() {
                    for b in cells.iter() {
                        if a == b {
                            continue;
                        }

                        let node_a = LinkNode::from(CellCandidate::from_cell(a, val));
                        let node_b = LinkNode::from(CellCandidate::from_cell(b, val));

                        map.entry(node_a).or_default().insert(node_b);
                    }
                }
            }
        }
    }

    map
}
