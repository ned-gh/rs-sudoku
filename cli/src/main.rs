use std::{
    fs::{self, File},
    io::{
        prelude::*,
        LineWriter,
    },
    sync::{Arc, Mutex},
    time::SystemTime,
};

use threadpool::ThreadPool;
use lib::{grid, solver, translator};

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

    let unsolved = Arc::new(Mutex::new(vec![]));

    let start_time = SystemTime::now();

    for (i, &path) in paths.iter().enumerate() {
        let contents = fs::read_to_string(path).unwrap();

        for line in contents.lines() {
            let line = translator::from_sudoku_exchange_bank_str(line).unwrap();
            let total_visited_clone = Arc::clone(&total_visited);
            let results_clone = Arc::clone(&results);
            let unsolved_clone = Arc::clone(&unsolved);

            pool.execute(move || {
                if solve_until_end(&line) {
                    results_clone.lock().unwrap()[i] += 1;
                } else {
                    unsolved_clone.lock().unwrap().push(line);
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

    let end_time = start_time.elapsed();

    let file = File::create("unsolved.txt").unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(unsolved.lock().unwrap().join("\n").as_bytes()).unwrap();
    file.flush().unwrap();

    match end_time {
        Ok(elapsed) => println!("Finished in {:.2}s", elapsed.as_secs_f64()),
        Err(e) => println!("Error while computing time: {:?}", e),
    };

    println!("Results:");
    for (i ,(solved, total)) in results.lock().unwrap().iter().zip(totals.iter()).enumerate() {
        let percentage_solved = 100.0 * (*solved as f64) / (*total as f64);
        println!("{} : {}/{} ({:.2}%) solved", paths[i], solved, total, percentage_solved);
    }
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
