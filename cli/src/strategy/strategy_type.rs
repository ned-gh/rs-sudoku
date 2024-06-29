use super::*;
use crate::grid::Grid;

pub enum Strategy {
    NakedSingle,
    HiddenSingle,
    PointingSet,
    LockedCandidates,
    NakedSet,
    HiddenSet,
}

use Strategy::*;

impl Strategy {
    pub fn get_finder(&self) -> fn(&Grid) -> Option<StrategyResult> {
        match self {
            NakedSingle => find_naked_single,
            HiddenSingle => find_hidden_single,
            PointingSet => find_pointing_set,
            LockedCandidates => find_locked_candidates,
            NakedSet => find_naked_set,
            HiddenSet => find_hidden_set,
        }
    }
}
