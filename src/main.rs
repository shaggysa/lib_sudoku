use std::{env, fs};
use std::io::{stdout, Write};
use tokio;

struct Puzzle {
    puzz: [u8; 81],
    blank_positions: Vec<u8>,
    possibilities: Vec<Vec<u8>>,
    cached_possibilities: Vec<[bool;10]>,
    current_pos: Vec<i8>,
    solved: bool,

}
#[derive(Clone)]
struct Puzzles {
    size: usize,
    unsolved: Vec<[u8; 81]>,
    solved: Vec<[u8; 81]>,
}
fn get_puzzles(file: String) -> Puzzles {
    let mut p = Puzzles {
        size: 0,
        unsolved: vec![],
        solved: vec![],
    };
    let puzzles_string = fs::read_to_string(file);
    let mut first_line = true;
    for line in puzzles_string.unwrap().lines() {
        if first_line {
            first_line = false;
            continue;
        }
        p.size += 1;
        let mut unsolved_list:[u8;81] = [0;81];
        let mut solved_list: [u8;81] = [0;81];
        let line_bytes = line.as_bytes();

        for num in 0..81 {
            unsolved_list[num] = line_bytes[num] - '0' as u8;
            solved_list[num] = line_bytes[num+82] - '0' as u8;
        }

        p.unsolved.push(unsolved_list);
        p.solved.push(solved_list);

    }
    p
}

fn get_unsolved_puzz(puzzles:&Puzzles, line_num:usize) -> [u8; 81] {
    puzzles.unsolved[line_num-2]
}

fn get_solved_puzz(puzzles:&Puzzles, line_num:usize) -> [u8; 81]{
    puzzles.solved[line_num-2]
}

fn get_possibilities(puzz: &[u8; 81], pos:u8) -> Vec<u8> {
    let mut possibilities:Vec<u8> = Vec::new();
    let mut seen: [bool;10] = [false;10];
    let row: usize = (pos / 9) as usize;
    let col: usize = (pos % 9) as usize;
    let root_row = (row/3) * 3;
    let root_col = (col/3) * 3;

    for r in (root_row..root_row + 3) {
        for c in root_col..root_col + 3 {
            seen[puzz[r * 9 + c] as usize] = true;
        }
    }
    for i in 0..9 {
        seen[puzz[i + (row*9)] as usize] = true;
        seen[puzz[col+(i*9)] as usize] = true;
    }

    for i in 1..10 {
        if !seen[i as usize] {
            possibilities.push(i)
        }
    }
    possibilities
}
fn solver_prep(puzz:[u8;81]) -> Puzzle {
    let mut p: Puzzle = Puzzle {puzz, blank_positions: Vec::new(), possibilities: Vec::new(), cached_possibilities: Vec::new(), current_pos: Vec::new(), solved: false};
    for i in 0..81 {
        if p.puzz[i] == 0 {
            p.blank_positions.push(i as u8);
        }
    }
    loop {
        let mut to_remove:Vec<u8> = Vec::new();
        let mut progressed = false;
        for i in p.blank_positions.iter().rev() {
            let possibilities = get_possibilities(&p.puzz, *i);
            if possibilities.len() == 1 {
                p.puzz[*i as usize] = possibilities[0];
                to_remove.push(*i);
                progressed = true;
            }
        }
        if progressed {
            p.blank_positions.retain(|&x| {
                if let Some(&last_item) = to_remove.last() {
                    if x == last_item {
                        to_remove.pop();
                        return false
                    }
                }
                true
            });
        } else {
            if p.blank_positions.len() == 0 {
                p.solved = true;
            } else {
                for i in 0..p.blank_positions.len() {
                    p.possibilities.push(get_possibilities(&p.puzz, p.blank_positions[i]));
                    p.current_pos.push(-1);
                }
            }
        break;
        }
    }
        p
}

fn get_possibilities_as_array(puzz : &[u8; 81], pos : usize) -> [bool; 10] {
    let mut seen: [bool;10] = [true;10];
    let row: usize = pos / 9;
    let col: usize = pos % 9;
    let root_row = (row/3) * 3;
    let root_col = (col/3) * 3;

    for r in root_row..root_row + 3 {
        for c in root_col..root_col + 3 {
            seen[puzz[r * 9 + c] as usize] = false;
        }
    }
    for i in 0..9 {
        seen[puzz[i + (row*9)] as usize] = false;
        seen[puzz[col+(i*9)] as usize] = false;
    }

    seen
}

fn solve(puzz : [u8;81]) -> [u8;81] {
    let mut p = solver_prep(puzz);
    if p.solved {
        return p.puzz
    }

    let mut position: i8 = 0;
    let max_pos = p.blank_positions.len();
    let mut progressed_forward = true;
    loop {
        if position < 0 {
            println!("Puzzle is unsolvable!");
            return [0; 81]
        } else if position == max_pos as i8 {
            p.solved = true;
            return p.puzz
        }
        let pos_usize = position as usize;
        let spot = p.blank_positions[pos_usize];
        let max = p.possibilities[pos_usize].len() - 1;
        let mut failed = true;
        if progressed_forward {
            if pos_usize == p.cached_possibilities.len() {
                p.cached_possibilities.push(get_possibilities_as_array(&p.puzz, spot as usize));
            } else {
                p.cached_possibilities[pos_usize] = get_possibilities_as_array(&p.puzz, spot as usize);
            }
        }
        while p.current_pos[pos_usize] < max as i8 {
            p.current_pos[pos_usize] += 1;
            if p.cached_possibilities[pos_usize][p.possibilities[pos_usize][p.current_pos[pos_usize] as usize] as usize] {
                p.puzz[spot as usize] = p.possibilities[pos_usize][p.current_pos[pos_usize] as usize];
                failed = false;
                position += 1;
                progressed_forward = true;
                break;
            }
        }
        if failed {
            p.puzz[spot as usize] = 0;
            p.current_pos[pos_usize] = -1;
            position -= 1;
            progressed_forward = false;
        }
    }
}

async fn async_solve(puzz : [u8;81]) -> [u8;81] {
    solve(puzz)
}
fn print_puzz(puzz : &[u8]) {
    let mut lock = stdout().lock();

    for row in 0..9 {
        for col in 0..9 {
            write!(lock, "{}  ", puzz[row * 9 + col]).unwrap();
        }
        writeln!(lock).unwrap();
    }
    lock.flush().unwrap();
}

async fn tokio_speedtest(puzzles: Puzzles) {
    println!("---------------\nStarting Async (tokio) Speedtest\n---------------");
    let start_solve = std::time::Instant::now();

    let handles: Vec<_> = puzzles.unsolved.into_iter().map(|puzz| tokio::spawn(async_solve(puzz))).collect();

    let mut solved_puzzles = Vec::new();
    for handle in handles {
        solved_puzzles.push(handle.await.unwrap());
    }
    let solve_time = start_solve.elapsed();
    for i in 0..solved_puzzles.len() {
        if solved_puzzles[i] != puzzles.solved[i] {
            println!("Solver Failed!");
            break
        }
    }
    println!("Solved {} Puzzles in {:?}", puzzles.size, solve_time);
    }


fn synchronous_speedtest(puzzles : Puzzles) {
    println!("---------------\nStarting Synchronous Speedtest\n---------------");
    let mut solved_puzzles:Vec<[u8;81]> = Vec::new();
    let start_solve = std::time::Instant::now();
    for i in 0..puzzles.unsolved.len() {
        solved_puzzles.push(solve(puzzles.unsolved[i]));
    }
    let solve_time = start_solve.elapsed();
    for i in 0..solved_puzzles.len() {
        if solved_puzzles[i] != puzzles.solved[i] {
            println!("Solver Failed!");
            break;
        }
    }
    println!("Solved {} Puzzles in {:?}", puzzles.size, solve_time);
}

#[tokio::main]
async fn main() {
    let puzzles: Puzzles;
    if env::args().count() < 2 {
        println!("No puzzle file argument found, using default.");
        let start_time = std::time::Instant::now();
        puzzles = get_puzzles("puzzles.csv".parse().unwrap());
        println!("Loaded {} puzzles in {:?}", puzzles.size, start_time.elapsed());
    } else {
        let start_time = std::time::Instant::now();
        puzzles = get_puzzles(env::args().nth(1).unwrap());
        println!("Loaded {} puzzles in {:?}", puzzles.size, start_time.elapsed());
    }
    tokio_speedtest(puzzles.clone()).await;
    synchronous_speedtest(puzzles);
}


