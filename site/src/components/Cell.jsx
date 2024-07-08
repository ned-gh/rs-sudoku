import "./Cell.css";

function Cell({ grid, idx, id, selected, onClick }) {
  function hasCandidate(n) {
    return ((grid.candidates[idx] >> n) & 1) == 0;
  };

  return (
    <div key={id} className="cell" onClick={onClick} style={{backgroundColor: (idx == selected) ? "cyan" : ""}}>
      {
        (grid.candidates[idx] != 0) ?
          [1,2,3,4,5,6,7,8,9].map((n) => {
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
