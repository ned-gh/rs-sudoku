use crate::grid::Grid;
use crate::strategy::*;

use Strategy::*;

pub struct Solver {
    grid: Grid,
    strategies: Vec<Strategy>,
}

impl Solver {
    pub fn from(grid: Grid) -> Solver {
        let strategies = vec![
            NakedSingle,
            HiddenSingle,
            PointingSet,
            LockedCandidates,
            NakedSet,
            HiddenSet,
        ];

        Solver { grid, strategies }
    }

    pub fn step(&self) -> Option<StrategyResult> {
        for strat in self.strategies.iter() {
            if let Some(result) = strat.get_finder()(&self.grid) {
                return Some(result);
            } else {
                continue;
            }
        }

        None
    }

    pub fn apply(&mut self, strategy_result: &StrategyResult) {
        for cell_candidate in strategy_result.get_to_place().iter() {
            self.grid.place(cell_candidate);
        }

        for cell_candidate in strategy_result.get_to_eliminate().iter() {
            self.grid.clear_candidate(cell_candidate);
        }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}
