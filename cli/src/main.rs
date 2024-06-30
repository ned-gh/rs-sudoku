use threadpool::ThreadPool;
use lib::{grid, solver, translator};

use std::{
    fs,
    sync::{Arc, Mutex},
};

fn main() {
    let paths = [
        "../sudoku-exchange-puzzle-bank/easy.txt",
        "../sudoku-exchange-puzzle-bank/medium.txt",
        "../sudoku-exchange-puzzle-bank/hard.txt",
        "../sudoku-exchange-puzzle-bank/diabolical.txt",
    ];

    let pool = ThreadPool::new(12);
    let results = Arc::new(Mutex::new(vec![0; paths.len()]));
    let total_visited = Arc::new(Mutex::new(0));
    let totals = paths.map(|path| fs::read_to_string(path).unwrap().lines().count());
    let grand_total: usize = totals.iter().sum();

    for (i, &path) in paths.iter().enumerate() {
        let contents = fs::read_to_string(path).unwrap();

        for line in contents.lines() {
            let line = translator::from_sudoku_exchange_bank_str(line).unwrap();
            let total_visited_clone = Arc::clone(&total_visited);
            let results_clone = Arc::clone(&results);

            pool.execute(move || {
                if solve_until_end(&line) {
                    results_clone.lock().unwrap()[i] += 1;
                }

                *total_visited_clone.lock().unwrap() += 1;

                let num = *total_visited_clone.lock().unwrap();
                if num % 10000 == 0 {
                    println!("Progress: {}/{}", num, grand_total);
                }
            });
        }
    }
    
    pool.join();

    println!("Results:");
    for (i ,(solved, total)) in results.lock().unwrap().iter().zip(totals.iter()).enumerate() {
        let percentage_solved = 100.0 * (*solved as f64) / (*total as f64);
        println!("{} : {}/{} ({:.2}%) solved", paths[i], solved, total, percentage_solved);
    }
}

fn solve_chunk(chunk: &[&str]) -> i32 {
    let mut solved = 0;
    for &bd in chunk {
        if solve_until_end(bd) {
            solved += 1;
        }
    }

    solved
}

fn solve_until_end(bd: &str) -> bool {
    let grid = grid::Grid::from_str(&bd).unwrap();
    let mut solver = solver::Solver::from(grid);

    loop {
        let Some(res) = solver.step() else {
            break;
        };

        solver.apply(&res);
    }

    solver.get_grid().is_complete()
}
