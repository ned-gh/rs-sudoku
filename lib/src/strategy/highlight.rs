use std::string::ToString;

use crate::grid::CellCandidate;

#[derive(Debug, Clone, Copy)]
pub enum HighlightColor {
    ElimFg,
    ElimBg,
    NoteFg,
    NoteBg,
    NoteNegativeFg,
    NoteNegativeBg,
    DefaultLineFg,
    Black,
    Red,
    Green,
    Blue,
    Cyan,
    Orange,
    Magenta,
    Yellow,
}

use HighlightColor::*;

impl ToString for HighlightColor {
    fn to_string(&self) -> String {
        match self {
            ElimFg         => "red".to_string(),
            ElimBg         => "yellow".to_string(),
            NoteFg         => "black".to_string(),
            NoteBg         => "green".to_string(),
            NoteNegativeFg => "black".to_string(),
            NoteNegativeBg => "green".to_string(),
            DefaultLineFg  => "grey".to_string(),
            Black          => "black".to_string(),
            Red            => "red".to_string(),
            Green          => "green".to_string(),
            Blue           => "blue".to_string(),
            Cyan           => "cyan".to_string(),
            Orange         => "orange".to_string(),
            Magenta        => "magenta".to_string(),
            Yellow         => "yellow".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Highlight {
    CellHighlight{row: u32, col: u32, bg: String},
    CandidateHighlight{cell_candidate: CellCandidate, fg: String, bg: String},
    LineHighlight{start: CellCandidate, end: CellCandidate, fg: String, dashed: bool},
}

use Highlight::{CellHighlight,CandidateHighlight, LineHighlight};

impl Highlight {
    pub fn new_cell_hl(row: u32, col: u32, bg: HighlightColor) -> Highlight {
        CellHighlight {
            row,
            col,
            bg: bg.to_string(),
        }
    }

    pub fn new_candidate_hl(cell_candidate: &CellCandidate, fg: HighlightColor, bg: HighlightColor) -> Highlight {
        CandidateHighlight {
            cell_candidate: cell_candidate.clone(),
            fg: fg.to_string(),
            bg: bg.to_string(),
        }
    }

    pub fn new_line_hl(start: &CellCandidate, end: &CellCandidate, fg: HighlightColor, dashed: bool) -> Highlight {
        LineHighlight {
            start: start.clone(),
            end: end.clone(),
            fg: fg.to_string(),
            dashed,
        }
    }
}
