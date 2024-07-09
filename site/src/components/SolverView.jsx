import { useState } from "react";
import * as wasm from "wasm";
import "./SolverView.css";

function SolverView({ gridHistory, setGridHistory, gridIdx, setGridIdx }) {
  const [steps, setSteps] = useState([{ name: "Start", idx: 0 }]);

  function updateGridHistory(newGrid) {
    const newGridHistory = gridHistory.slice();
    newGridHistory.push(newGrid);
    
    setGridHistory(newGridHistory);
    setGridIdx(newGridHistory.length - 1);
  }

  function addStep(strategyResult) {
    const newSteps = steps.slice();
    newSteps.push(
      {
        name: strategyResult.name,
        idx: gridHistory.length - 1,
      }
    );

    setSteps(newSteps);
  }

  function stepSolver() {
    const grid = gridHistory[gridHistory.length - 1];

    const result = wasm.solve_step(grid);

    const [strategyResult, newGrid] = result;

    if (newGrid == undefined) {
      if (steps.length <= gridHistory.length) {
        addStep({ name: "Completed", idx: gridHistory.length - 1 });
      }
      return;
    }

    addStep(strategyResult);
    updateGridHistory(newGrid);
  }

  return (
    <div className="solver-view">
      <div className="solver-buttons">
        <button onClick={stepSolver}>Step</button>
      </div>
      <div className="solver-steps">
        <ul>
          {
            steps.map((step) => {
              return <li key={step.idx.toString() + step.name} onClick={() => setGridIdx(step.idx)}>{step.name}</li>;
            })
          }
        </ul>
      </div>
    </div>
  );
}

export default SolverView;
