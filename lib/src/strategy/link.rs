use std::collections::{HashMap, HashSet};

use crate::grid::{Cell, CellCandidate, Grid, UnitType};

use UnitType::{Col, MiniGrid, Row};

pub type LinkMap = HashMap<CellCandidate, HashSet<CellCandidate>>;

pub enum LinkType {
    StrongInCell,
    StrongInUnit,
    WeakInCell,
    WeakInUnit,
}

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

    let cells: Vec<Cell> = if only_strong_links {
        grid.get_nvalue_cells(2).iter().cloned().collect()
    } else {
        grid.iter().collect()
    };

    for cell in cells.iter() {
        let candidates = cell.get_candidates();

        for a in candidates.iter() {
            for b in candidates.iter() {
                if a == b {
                    continue;
                }

                let cell_a = CellCandidate::from_cell(cell, a);
                let cell_b = CellCandidate::from_cell(cell, b);

                if !map.contains_key(&cell_a) {
                    map.insert(cell_a.clone(), HashSet::new());
                };

                map.get_mut(&cell_a).unwrap().insert(cell_b);
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

                        let cell_a = CellCandidate::from_cell(a, val);
                        let cell_b = CellCandidate::from_cell(b, val);

                        if !map.contains_key(&cell_a) {
                            map.insert(cell_a.clone(), HashSet::new());
                        };

                        map.get_mut(&cell_a).unwrap().insert(cell_b);
                    }
                }
            }
        }
    }

    map
}
