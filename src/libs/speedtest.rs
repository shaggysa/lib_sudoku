use crate::libs::puzzle_reader;
use crate::libs::puzzle_solver::{solve, async_solve};
pub async fn tokio_speedtest(puzzle_reader: puzzle_reader::PuzzleReader) {
    println!("---------------\nStarting Async (tokio) Speedtest\n---------------");
    let start_solve = std::time::Instant::now();

    let handles: Vec<_> = puzzle_reader.unsolved.into_iter().map(|puzz| tokio::spawn(async_solve(puzz))).collect();

    let mut solved_puzzles = Vec::new();
    for handle in handles {
        solved_puzzles.push(handle.await.unwrap());
    }
    let solve_time = start_solve.elapsed();
    for i in 0..solved_puzzles.len() {
        if solved_puzzles[i] != puzzle_reader.solved[i] {
            println!("Solver Failed!");
            break
        }
    }
    println!("Solved {} Puzzles in {:?}", puzzle_reader.size, solve_time);
}


pub fn synchronous_speedtest(puzzle_reader: puzzle_reader::PuzzleReader) {
    println!("---------------\nStarting Synchronous Speedtest\n---------------");
    let mut solved_puzzles:Vec<[u8;81]> = Vec::new();
    let start_solve = std::time::Instant::now();
    for i in 0..puzzle_reader.unsolved.len() {
        solved_puzzles.push(solve(puzzle_reader.unsolved[i]));
    }
    let solve_time = start_solve.elapsed();
    for i in 0..solved_puzzles.len() {
        if solved_puzzles[i] != puzzle_reader.solved[i] {
            println!("Solver Failed!");
            break;
        }
    }
    println!("Solved {} Puzzles in {:?}", puzzle_reader.size, solve_time);
}