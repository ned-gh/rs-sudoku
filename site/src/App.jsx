import './App.css'

import * as wasm from "wasm";

function App() {
  const bd = "300967001040302080020000070070000090000873000500010003004705100905000207800621004";
  const grid = wasm.get_grid_from_bd_str(bd);
  console.log(grid);

  return (
    <div>
      <h1>Hello, world!</h1>
    </div>
  )
}

export default App
