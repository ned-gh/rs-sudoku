import { useState } from "react";
import Sudoku from "./components/Sudoku";
import SolverView from "./components/SolverView";
import * as wasm from "wasm";
import "./App.css";

const bd = "300967001040302080020000070070000090000873000500010003004705100905000207800621004";

function App() {
  const [gridIdx, setGridIdx] = useState(0);
  const [gridHistory, setGridHistory] = useState([wasm.get_grid_from_bd_str(bd)]);
  const [selected, setSelected] = useState(null);

  return (
    <div className="app">
      <Sudoku
        gridIdx={gridIdx}
        setGridIdx={setGridIdx}
        gridHistory={gridHistory}
        setGridHistory={gridHistory}
        selected={selected}
        setSelected={setSelected}
      />
      <SolverView
        gridIdx={gridIdx}
        setGridIdx={setGridIdx}
        gridHistory={gridHistory}
        setGridHistory={setGridHistory}
      />
    </div>
  )
}

export default App;
