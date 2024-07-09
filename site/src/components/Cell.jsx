import "./Cell.css";

function Cell({ grid, idx, selected, highlighter, onClick }) {
  function hasCandidate(n) {
    return ((grid.candidates[idx] >> n) & 1) == 1;
  };

  const cellBg = highlighter == null ? "" : (highlighter.getCellBg(idx) == null ? "" : highlighter.getCellBg(idx));

  return (
    <div className="cell" onClick={onClick} style={{backgroundColor: (idx == selected) ? "cyan" : cellBg}}>
      {
        (grid.candidates[idx] != 0) ?
          [1,2,3,4,5,6,7,8,9].map((n) => {
            if (highlighter != null) {
              const num = 82 + idx*9 + n;
              const fg = highlighter.getCandidateFg(num);
              const bg = highlighter.getCandidateBg(num);

              return (
                <div className="candidate" key={82 + idx*9 + n} style={{color: fg, backgroundColor: bg}}>
                  {hasCandidate(n) && n}
                </div>
              );
            }
            return (
              <div className="candidate" key={82 + idx*9 + n}>
                {hasCandidate(n) && n}
              </div>
            );
          }) :
          (grid.placed[idx] != 0) && grid.placed[idx]
      }
    </div>
  );
}

export default Cell;
