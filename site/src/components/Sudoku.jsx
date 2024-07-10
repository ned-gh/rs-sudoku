import { useEffect, useState, useRef } from "react";
import Xarrow from "react-xarrows";
import "./Sudoku.css";

function Sudoku({ gridIdx, gridHistory, setSelected, highlights }) {
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
  
  const ref = useOutsideClick(() => setSelected(null));

  const refs = useRef({});

  function getGrid() {
    const grid = gridHistory[gridIdx];

    const elements = [];

    for (let r = 0; r < 9; r++) {
      const row = [];

      for (let c = 0; c < 9; c++) {
        const i = 9*r + c;
        const cellId = getCellId(i);

        let content;

        if (grid.placed[i] != 0) {
          content = grid.placed[i];
        } else {
          content = [1,2,3,4,5,6,7,8,9].map((n) => {
            const candidateId = getCandidateId(i, n);
            return (
              <div
              key={candidateId}
              id={candidateId}
              className="candidate"
              ref={(e) => {
                if (e) {
                  refs.current[candidateId] = e;
                } else {
                  delete refs.current[candidateId];
                }
              }}
              style={{ backgroundColor: "" }}
              >
              {(((grid.candidates[i] >> n) & 1) == 1) && n}
              </div>
            );
          });
        }

        row.push(
          <div
          key={cellId}
          id={cellId}
          className="cell"
          ref={(e) => {
            if (e) {
              refs.current[cellId] = e;
            } else {
              delete refs.current[cellId];
            }
          }}
          style={{ backgroundColor: "" }}
          >
          {content}
          </div>
        );
      }

      elements.push(
        <div key={1000 + r} className="row">{row}</div>
      );
    }

    return elements;
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

  const [arrows, setArrows] = useState([]);

  useEffect(() => {
    for (const key in refs.current) {
      refs.current[key].style.color = "";
      refs.current[key].style.backgroundColor = "";
    }

    if (highlights == null) {
      return;
    }

    const newArrows = [];

    for (let i = 0; i < highlights.length; i++) {
      const highlight = highlights[i];

      if (highlight.CandidateHighlight != undefined) {
        const hl = highlight.CandidateHighlight;

        const row = hl.cell_candidate.row;
        const col = hl.cell_candidate.col;
        const val = hl.cell_candidate.val;
        const id = getCandidateId(9*row + col, val);

        const fg = colorMap(hl.fg);
        const bg = colorMap(hl.bg);

        refs.current[id].style.color = fg;
        refs.current[id].style.backgroundColor = bg;
      } else if (highlight.CellHighlight != undefined) {
        const hl = highlight.CellHighlight;

        const id = getCellId(9*hl.row + hl.col);

        const bg = colorMap(hl.bg);

        refs.current[id].style.backgroundColor = bg;
      } else if (highlight.LineHighlight != undefined) {
        const hl = highlight.LineHighlight;

        const start = hl.start;
        const end = hl.end;

        const startId = getCandidateId(9*start.row + start.col, start.val);
        const endId = getCandidateId(9*end.row + end.col, end.val);

        const fg = hl.fg;
        const dashed = hl.dashed;

        newArrows.push([startId, endId, fg, dashed]);
      }
    }

    setArrows(newArrows);
  }, [highlights]);

  return (
    <>
      <div className="sudoku" ref={ref}>
        {getGrid()}
      </div>
      <div className="arrows">
      {
        arrows.map((arr) => {
          return (
            <Xarrow
              key={1024+arr[0]*9+arr[1]}
              start={arr[0].toString()}
              end={arr[1].toString()}
              color={arr[2]}
              dashness={arr[3] ? {strokeLen: 10, nonStrokeLen: 5} : false}
              strokeWidth={2}
              headSize={3}
              path="smooth"
              curveness={0.5}
            />
          );
        })
      }
      </div>
    </>
  )
}

function getCellId(i) {
  return i;
}

function getCandidateId(i, n) {
  return 81 + i*9 + n - 1;
}

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

export default Sudoku;
