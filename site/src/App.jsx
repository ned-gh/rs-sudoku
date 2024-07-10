import { useState } from "react";
import Sudoku from "./components/Sudoku";
import SolverView from "./components/SolverView";
import * as wasm from "wasm";
import "./App.css";

const DEFAULT_BD = "908020076000000100070000020005400091300702005460005800040000050006000000210070304";

function App() {
  const [gridIdx, setGridIdx] = useState(0);
  const [gridHistory, setGridHistory] = useState([wasm.get_grid_from_bd_str(DEFAULT_BD)]);
  const [selected, setSelected] = useState(null);
  const [highlights, setHighlights] = useState(null);
  const [bd, setBd] = useState("");

  function setGridToBd() {
    const grid = wasm.get_grid_from_bd_str(bd);

    if (grid != undefined) {
      setGridIdx(0);
      setGridHistory([grid]);
      setSelected(null);
      setHighlights(null);
    }
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
          gridHistory={gridHistory}
          setSelected={setSelected}
          highlights={highlights}
        />
        <SolverView
          gridHistory={gridHistory}
          setGridHistory={setGridHistory}
          setGridIdx={setGridIdx}
          setHighlights={setHighlights}
        />
      </div>
    </div>
  )
}

export default App;
