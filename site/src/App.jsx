import { useState } from "react";
import Sudoku from "./components/Sudoku";
import SolverView from "./components/SolverView";
import * as wasm from "wasm";
import "./App.css";

const bd = "300009000000001020056002790003000048000040107009000000000000000080360200500820400";

function colorMap(color) {
  switch (color) {
    case "green":
      return "#77ff77";
    case "black":
      return "#000000";
    case "red":
      return "#ff7777";
    case "yellow":
      return "#ffff00";
  }

  return null;
}

function App() {
  const [gridIdx, setGridIdx] = useState(0);
  const [gridHistory, setGridHistory] = useState([wasm.get_grid_from_bd_str(bd)]);
  const [selected, setSelected] = useState(null);
  const [highlighter, setHighlighter] = useState(null);

  // highlights: {
  //   candidate_highlight: {
  //     row, col, val, fg, bg
  //   },
  //   cell_highlight: {
  //     row, col, bg
  //   },
  // }

  function setHighlights(highlights) {
    if (highlights == null) {
      setHighlighter(null);
      return;
    }

    const __highlights = {
      candidateHighlights: {},
    };
    
    for (let i = 0; i < highlights.length; i++) {
      const hl = highlights[i];

      if (hl["CandidateHighlight"] != undefined) {
        const cc = hl["CandidateHighlight"].cell_candidate;
        const fg = colorMap(hl["CandidateHighlight"].fg);
        const bg = colorMap(hl["CandidateHighlight"].bg);

        const key = 82 + 9*(9*cc.row + cc.col) + cc.val;
        const value = {fg: fg, bg: bg};
        __highlights.candidateHighlights[key] = value;
      }
    }

    const getCandidateFg = (key) => {
      if (__highlights.candidateHighlights == undefined) {
        return null;
      }

      if (__highlights.candidateHighlights[key] != undefined) {
        return __highlights.candidateHighlights[key].fg;
      }

      return null;
    };

    const getCandidateBg = (key) => {
      if (__highlights.candidateHighlights == undefined) {
        return null;
      }

      if (__highlights.candidateHighlights[key] != undefined) {
        return __highlights.candidateHighlights[key].bg;
      }

      return null;
    };

    setHighlighter({ __highlights, getCandidateFg, getCandidateBg });
  }

  return (
    <div className="app">
      <Sudoku
        gridIdx={gridIdx}
        setGridIdx={setGridIdx}
        gridHistory={gridHistory}
        setGridHistory={gridHistory}
        selected={selected}
        setSelected={setSelected}
        highlighter={highlighter}
      />
      <SolverView
        gridIdx={gridIdx}
        setGridIdx={setGridIdx}
        gridHistory={gridHistory}
        setGridHistory={setGridHistory}
        setHighlights={setHighlights}
      />
    </div>
  )
}

export default App;
