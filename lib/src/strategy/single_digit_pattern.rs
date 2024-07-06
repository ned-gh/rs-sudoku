use super::{
    aic::{build_aics, AICResult, AICType, AIC},
    highlight::{Highlight, HighlightColor},
    link::{make_link_map, LinkType},
    StrategyResult,
};
use crate::grid::{CellCandidate, Grid, Region, Unit};

use LinkType::{StrongInUnit, WeakInUnit};
use Unit::{Col, Row};

enum PatternType {
    Skyscraper,
    TwoStringKite,
    TurbotFish,
}

impl PatternType {
    fn to_str(&self) -> &str {
        match self {
            Skyscraper => "Skyscraper",
            TwoStringKite => "2-String Kite",
            TurbotFish => "Turbot Fish",
        }
    }
}

use PatternType::{Skyscraper, TurbotFish, TwoStringKite};

pub fn find_single_digit_pattern(grid: &Grid) -> Option<StrategyResult> {
    let strong_link_map = make_link_map(grid, &[StrongInUnit]);
    let weak_link_map = make_link_map(grid, &[StrongInUnit, WeakInUnit]);

    if let Some(aic_result) = build_aics(&strong_link_map, &weak_link_map, 4) {
        let pattern_type = match aic_result.get_aic_type() {
            AICType::Continuous => TurbotFish,
            AICType::Discontinuous => get_pattern_type(grid, aic_result.get_aic()),
        };

        let highlights = make_higlights(grid, &pattern_type, &aic_result);

        return Some(StrategyResult::from(
            pattern_type.to_str(),
            aic_result.get_to_place().clone(),
            aic_result.get_to_eliminate().clone(),
            highlights,
        ));
    }

    None
}

fn get_pattern_type(grid: &Grid, aic: &AIC) -> PatternType {
    let first = Region::from_candidates(grid, &aic[0..=1]);
    let middle = Region::from_candidates(grid, &aic[1..=2]);
    let last = Region::from_candidates(grid, &aic[2..=3]);

    match (first.all_in_line(), last.all_in_line()) {
        (Some(Row(_)), Some(Row(_))) | (Some(Col(_)), Some(Col(_))) => {
            if middle.all_in_line().is_some() {
                Skyscraper
            } else {
                TurbotFish
            }
        }
        (Some(Row(_)), Some(Col(_))) | (Some(Col(_)), Some(Row(_))) => {
            if middle.all_in_minigrid().is_some() {
                TwoStringKite
            } else {
                TurbotFish
            }
        }
        _ => TurbotFish,
    }
}

fn make_higlights(
    grid: &Grid,
    pattern_type: &PatternType,
    aic_result: &AICResult,
) -> Vec<Highlight> {
    let mut highlights = match pattern_type {
        TurbotFish => aic_result.make_highlights(true, true),
        _ => aic_result.make_highlights(false, false),
    };

    for cell_candidate in aic_result.get_to_eliminate().iter() {
        highlights.push(Highlight::new_candidate_hl(
            cell_candidate,
            HighlightColor::ElimFg,
            HighlightColor::ElimBg,
        ));
    }

    for cell_candidate in aic_result.get_to_place().iter() {
        let (r, c, val) = cell_candidate.as_tuple();

        for elim_val in grid.get_candidates(r, c).iter() {
            if elim_val == val {
                continue;
            }

            highlights.push(Highlight::new_candidate_hl(
                &CellCandidate::from(r, c, elim_val),
                HighlightColor::ElimFg,
                HighlightColor::ElimBg,
            ));
        }
    }

    highlights
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::CellCandidate;

    #[test]
    fn test_skyscraper() {
        let bd =
            "697000002001972063003006790912000607374260950865709024148693275709024006006807009";
        let mut grid = Grid::from_str(bd).unwrap();

        // remove turbot fishes
        grid.clear_candidate(&CellCandidate::from(8, 4, 1));
        grid.clear_candidate(&CellCandidate::from(5, 4, 1));
        grid.clear_candidate(&CellCandidate::from(0, 4, 1));

        // remove 2-string kite
        grid.clear_candidate(&CellCandidate::from(5, 6, 1));

        let expected = vec![
            CellCandidate::from(0, 6, 1),
            CellCandidate::from(0, 7, 1),
            CellCandidate::from(2, 3, 1),
            CellCandidate::from(2, 4, 1),
        ];

        let skyscraper = find_single_digit_pattern(&grid).unwrap();
        let to_place = skyscraper.get_to_place().clone();
        let mut to_eliminate = skyscraper.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!("Skyscraper", skyscraper.get_name());
        assert_eq!(expected, to_eliminate);
        assert_eq!(Vec::<CellCandidate>::new(), to_place);
    }

    #[test]
    fn test_two_string_kite() {
        let bd =
            "081020600042060089056800240693142758428357916175689324510036892230008460860200000";
        let mut grid = Grid::from_str(bd).unwrap();

        // remove turbot fish
        grid.clear_candidate(&CellCandidate::from(0, 8, 5));
        grid.clear_candidate(&CellCandidate::from(8, 5, 5));

        // remove skyscraper
        grid.clear_candidate(&CellCandidate::from(8, 8, 5));

        let expected = vec![CellCandidate::from(1, 3, 5)];

        let skyscraper = find_single_digit_pattern(&grid).unwrap();
        let to_place = skyscraper.get_to_place().clone();
        let mut to_eliminate = skyscraper.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!("2-String Kite", skyscraper.get_name());
        assert_eq!(expected, to_eliminate);
        assert_eq!(Vec::<CellCandidate>::new(), to_place);
    }

    #[test]
    fn test_turbot_fish() {
        let bd =
            "700054010063870425504000700270400001400920007000007542852043079390782054047590283";
        let mut grid = Grid::from_str(bd).unwrap();

        // remove turbot fishes
        grid.clear_candidate(&CellCandidate::from(4, 6, 8));
        grid.clear_candidate(&CellCandidate::from(3, 6, 9));
        grid.clear_candidate(&CellCandidate::from(0, 6, 9));
        grid.clear_candidate(&CellCandidate::from(0, 2, 9));
        grid.clear_candidate(&CellCandidate::from(1, 0, 1));
        grid.clear_candidate(&CellCandidate::from(6, 3, 1));
        grid.clear_candidate(&CellCandidate::from(0, 8, 8));

        // remove 2-string kite
        grid.clear_candidate(&CellCandidate::from(2, 4, 1));

        // remove skyscraper
        grid.clear_candidate(&CellCandidate::from(5, 2, 9));

        let expected = vec![CellCandidate::from(5, 3, 6)];

        let skyscraper = find_single_digit_pattern(&grid).unwrap();
        let to_place = skyscraper.get_to_place().clone();
        let mut to_eliminate = skyscraper.get_to_eliminate().clone();

        to_eliminate.sort();

        assert_eq!("Turbot Fish", skyscraper.get_name());
        assert_eq!(expected, to_eliminate);
        assert_eq!(Vec::<CellCandidate>::new(), to_place);
    }
}
