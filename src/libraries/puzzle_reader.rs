use pyo3::{PyResult, pyclass, PyErr, pyfunction, pymethods};
use std::io::{Write, stdout};

#[pyclass]
#[derive(Clone)]
pub struct PuzzleReader {
    pub size: usize,
    pub unsolved: Vec<[u8; 81]>,
    pub solved: Vec<[u8; 81]>,
}

#[pymethods]
impl PuzzleReader {
    #[new]
    pub fn load_puzzles(file: &str, from_url: bool) -> PyResult<Self> {



        let puzzles_string: String;

        if from_url {
            let download_start = std::time::Instant::now();
            puzzles_string = ureq::get(file)
                .call()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
                .body_mut()
                .read_to_string().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            println!("Downloaded {} in {:?}", file, download_start.elapsed());
        }

        else {
            let read_start = std::time::Instant::now();
            puzzles_string = std::fs::read_to_string(&file)?;
            println!("Read {} in {:?}", file, read_start.elapsed());
        }

        let num_puzzles = (puzzles_string.len() - 15) * 81/82/(81*2);

        let mut p = PuzzleReader {
            size: num_puzzles,
            unsolved: Vec::with_capacity(num_puzzles),
            solved: Vec::with_capacity(num_puzzles),
        };

        let parse_start = std::time::Instant::now();
        let mut first_line = true;
        for line in puzzles_string.lines() {
            if first_line {
                first_line = false;
                continue;
            }
            let mut unsolved_list: [u8; 81] = [0; 81];
            let mut solved_list: [u8; 81] = [0; 81];
            let line_bytes = line.as_bytes();

            if line_bytes.len() == 163 && line_bytes[81] == 0x2c { //0x2c is the utf-8 char for a comma
                for num in 0..81 {
                    unsolved_list[num] = line_bytes[num] - '0' as u8;
                    solved_list[num] = line_bytes[num + 82] - '0' as u8;
                }

                p.unsolved.push(unsolved_list);
                p.solved.push(solved_list);
            } else {
                return Err::<Self, PyErr>(pyo3::exceptions::PyValueError::new_err(format!("Line {} in {} is malformed:{}", p.size+1, file, line)));
            }

        }
        println!("Parsed {} puzzles in {:?}.", p.size, parse_start.elapsed());
        Ok(p)
    }

    pub fn get_unsolved_puzz(&self, line_num: usize) -> PyResult<[u8; 81]> {
        Ok(self.unsolved[line_num - 2].clone())
    }

    pub fn get_solved_puzz(&self, line_num: usize) -> PyResult<[u8; 81]> {
        Ok(self.solved[line_num - 2].clone())
    }
}

#[pyfunction]
pub fn print_puzz(puzz: Vec<u8>) {
    backend_print_puzz(puzz.try_into().expect("A puzzle must have a length of 81!"));
}

pub fn backend_print_puzz(puzz: [u8; 81]) {
    let mut lock = stdout().lock();

    for row in 0..9 {
        for col in 0..9 {
            write!(lock, "{}  ", puzz[row * 9 + col]).unwrap();
        }
        writeln!(lock).unwrap();
    }
    lock.flush().unwrap();
}
