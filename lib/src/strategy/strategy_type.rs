use super::*;
use crate::grid::Grid;

#[derive(Debug)]
pub enum Strategy {
    NakedSingle,
    HiddenSingle,
    PointingSet,
    LockedCandidates,
    NakedSet,
    HiddenSet,
    XWing,
    XYWing,
    XYZWing,
    Swordfish,
    RectangleElimination,
    SingleDigitPattern,
    AIC,
}

use Strategy::*;

impl Strategy {
    pub fn get_all() -> Vec<Strategy> {
        vec![
            NakedSingle,
            HiddenSingle,
            PointingSet,
            LockedCandidates,
            NakedSet,
            HiddenSet,
            XWing,
            XYWing,
            XYZWing,
            Swordfish,
            RectangleElimination,
            SingleDigitPattern,
            AIC,
        ]
    }

    pub fn get_finder(&self) -> fn(&Grid) -> Option<StrategyResult> {
        match self {
            NakedSingle => find_naked_single,
            HiddenSingle => find_hidden_single,
            PointingSet => find_pointing_set,
            LockedCandidates => find_locked_candidates,
            NakedSet => find_naked_set,
            HiddenSet => find_hidden_set,
            XWing => find_xwing,
            XYWing => find_xywing,
            XYZWing => find_xyzwing,
            Swordfish => find_swordfish,
            RectangleElimination => find_rectangle_elimination,
            SingleDigitPattern => find_single_digit_pattern,
            AIC => find_general_aic,
        }
    }
}
