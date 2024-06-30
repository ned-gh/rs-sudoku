mod grid;
mod solver;
mod strategy;
mod translator;
mod util;

use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let paths = [
        "../sudoku-exchange-puzzle-bank/easy.txt",
        "../sudoku-exchange-puzzle-bank/medium.txt",
        "../sudoku-exchange-puzzle-bank/hard.txt",
        "../sudoku-exchange-puzzle-bank/diabolical.txt",
    ];

    let results = Arc::new(Mutex::new(vec![]));
    let mut join_handles = vec![];

    for path in paths {
        let results_clone = Arc::clone(&results);
        join_handles.push(thread::spawn(move || {
            let (solved, total) = solve_file(path);
            results_clone
                .lock()
                .unwrap()
                .push((solved, total, path.to_string()));
        }));
    }

    for jh in join_handles {
        let _ = jh.join();
    }

    println!("----------");
    println!("results:");
    for (solved, total, path_str) in results.lock().unwrap().iter() {
        println!("  {} : {}/{} solved", path_str, solved, total);
    }
}

fn solve_file(path: &str) -> (i32, i32) {
    let contents = fs::read_to_string(path).unwrap();

    let mut solved_count = 0;
    let mut total = 0;

    for line in contents.lines() {
        let bd = translator::from_sudoku_exchange_bank_str(line).unwrap();
        if solve_until_end(&bd) {
            solved_count += 1;
        }

        total += 1;

        if total % 1000 == 0 {
            println!("{} progress: {}/{} solved", path, solved_count, total);
        }
    }

    println!("solved {}/{} from {}", solved_count, total, path);

    (solved_count, total)
}

fn solve_until_end(bd: &str) -> bool {
    let grid = grid::Grid::from_str(bd).unwrap();
    let mut solver = solver::Solver::from(grid);

    loop {
        let Some(res) = solver.step() else {
            break;
        };

        solver.apply(&res);
    }

    solver.get_grid().is_complete()
}
