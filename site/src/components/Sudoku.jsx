import { useEffect, useRef } from "react";
import Cell from "./Cell";
import "./Sudoku.css";

function Sudoku({ gridIdx, setGridIdx, gridHistory, setGridHistory, selected, setSelected }) {
  function useOutsideClick(callback) {
    const ref = useRef();

    useEffect(() => {
      const handleClick = (e) => {
        if (ref.current && !ref.current.contains(e.target)) {
          callback();
        }
      };

      document.addEventListener("click", handleClick);

      return () => {
        document.removeEventListener("click", handleClick);
      };
    }, []);

    return ref;
  };
  
  function getRow(row) {
    const cells = [];

    for (let c = 0; c < 9; c++) {
      const idx = row*9 + c;
      cells.push(
        <Cell
          grid={gridHistory[gridIdx]}
          idx={idx}
          key={idx}
          selected={selected}
          onClick={() => setSelected(idx)}
        />
      );
    }

    return (
      <div className="row" key={730 + row}>
        {cells}
      </div>
    );
  };

  function getGrid() {
    const rows = [];

    for (let r = 0; r < 9; r++) {
      rows.push(getRow(r));
    }

    return rows;
  };

  const ref = useOutsideClick(() => setSelected(null));

  return (
    <div className="sudoku" ref={ref}>
      {getGrid()}
    </div>
  )
}

export default Sudoku;
