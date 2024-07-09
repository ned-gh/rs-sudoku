import { useState } from "react";
import Sudoku from "./components/Sudoku";
import SolverView from "./components/SolverView";
import * as wasm from "wasm";
import "./App.css";

const DEFAULT_BD = "908020076000000100070000020005400091300702005460005800040000050006000000210070304";

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
    case "orange":
      return "#eeaa55";
    case "magenta":
      return "magenta";
    case "cyan":
      return "cyan";
    case "blue":
      return "#7777ff";
    case "grey":
      return "grey";
  }

  return null;
}

function App() {
  const [gridIdx, setGridIdx] = useState(0);
  const [gridHistory, setGridHistory] = useState([wasm.get_grid_from_bd_str(DEFAULT_BD)]);
  const [selected, setSelected] = useState(null);
  const [highlighter, setHighlighter] = useState(null);
  const [bd, setBd] = useState("");

  function setGridToBd() {
    const grid = wasm.get_grid_from_bd_str(bd);

    if (grid != undefined) {
      setGridIdx(0);
      setGridHistory([grid]);
      setSelected(null);
      setHighlighter(null);
    }
  }

  // highlights: {
  //   CandidateHighlight: {
  //     cell_candidate: {row, col, val},
  //     fg,
  //     bg
  //   },
  //
  //   CellHighlight: {
  //     row,
  //     col,
  //     bg
  //   },
  //
  //   LineHighlight: {
  //     start: {row, col, val},
  //     end: {row, col, val},
  //     fg,
  //     dashed,
  //   }
  // }

  function setHighlights(highlights) {
    if (highlights == null) {
      setHighlighter(null);
      return;
    }

    const __highlights = {
      candidateHighlights: {},
      cellHighlights: {},
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
      } else if (hl["CellHighlight"] != undefined) {
        const row = hl["CellHighlight"].row;
        const col = hl["CellHighlight"].col;
        const bg = colorMap(hl["CellHighlight"].bg);

        const key = 9*row + col;
        const value = {bg: bg};
        __highlights.cellHighlights[key] = value;
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

    const getCellBg = (key) => {
      if (__highlights.cellHighlights == undefined) {
        return null;
      }

      if (__highlights.cellHighlights[key] != undefined) {
        return __highlights.cellHighlights[key].bg;
      }

      return null;
    }

    setHighlighter({ __highlights, getCandidateFg, getCandidateBg, getCellBg });
  }

  return (
    <div className="app">
      <div className="load-container">
        <input type="text" value={bd} onChange={(e) => setBd(e.target.value)} />
        <button onClick={() => setGridToBd()}>Load</button>
      </div>
      <div className="solver-container">
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
    </div>
  )
}

export default App;
