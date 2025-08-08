use std::io::{stdout, Write};

#[derive(Clone)]
pub struct PuzzleReader {
    pub(crate) size: usize,
    pub(crate) unsolved: Vec<[u8; 81]>,
    pub(crate) solved: Vec<[u8; 81]>,
}
impl PuzzleReader {
    pub fn load_puzzles(file: String) -> PuzzleReader {
        let mut p = PuzzleReader {
            size: 0,
            unsolved: vec![],
            solved: vec![],
        };
        let puzzles_string = std::fs::read_to_string(file);
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

    pub fn get_unsolved_puzz(&self, line_num:usize) -> [u8; 81] {
        self.unsolved[line_num-2]
    }

    pub fn get_solved_puzz(&self, line_num:usize) -> [u8; 81]{
        self.solved[line_num-2]
    }
}

pub fn print_puzz(puzz : &[u8]) {
    let mut lock = stdout().lock();

    for row in 0..9 {
        for col in 0..9 {
            write!(lock, "{}  ", puzz[row * 9 + col]).unwrap();
        }
        writeln!(lock).unwrap();
    }
    lock.flush().unwrap();
}
