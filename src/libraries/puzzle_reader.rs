use pyo3::{PyResult, pyclass, pyfunction, pymethods};
use std::io::{Write, stdout};

#[derive(Clone)]
#[pyclass]
pub struct PuzzleReader {
    pub size: usize,
    pub unsolved: Vec<Vec<u8>>,
    pub solved: Vec<Vec<u8>>,
}

#[pymethods]
impl PuzzleReader {
    #[new]
    pub fn load_puzzles(file: String) -> PyResult<Self> {
        let start = std::time::Instant::now();
        let mut p = PuzzleReader {
            size: 0,
            unsolved: Vec::new(),
            solved: Vec::new(),
        };
        let puzzles_string = std::fs::read_to_string(&file);
        let mut first_line = true;
        for line in puzzles_string?.lines() {
            if first_line {
                first_line = false;
                continue;
            }
            p.size += 1;
            let mut unsolved_list = vec![0;81];
            let mut solved_list = vec![0; 81];
            let line_bytes = line.as_bytes();

            if line_bytes.len() == 163 && line_bytes[81] == 0x2c { //0x2c is the utf-8 char for a comma
                for num in 0..81 {
                    unsolved_list[num] = line_bytes[num] - '0' as u8;
                    solved_list[num] = line_bytes[num + 82] - '0' as u8;
                }

                p.unsolved.push(unsolved_list);
                p.solved.push(solved_list);
            } else {
                return Err::<Self, pyo3::PyErr>(pyo3::exceptions::PyValueError::new_err(format!("Line {} in {} is malformed:{}", p.size+1, &file, line)));
            }

        }
        println!("Read {} puzzles in {:?}.", p.size, start.elapsed());
        Ok(p)
    }

    pub fn get_unsolved_puzz(&self, line_num: usize) -> PyResult<Vec<u8>> {
        Ok(self.solved[line_num - 2].clone())
    }

    pub fn get_solved_puzz(&self, line_num: usize) -> PyResult<Vec<u8>> {
        Ok(self.solved[line_num - 2].clone())
    }
}

#[pyfunction]
pub fn print_puzz(puzz: Vec<u8>) {
    let mut lock = stdout().lock();

    for row in 0..9 {
        for col in 0..9 {
            write!(lock, "{}  ", puzz[row * 9 + col]).unwrap();
        }
        writeln!(lock).unwrap();
    }
    lock.flush().unwrap();
}
