use std::fs;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

struct puzzles {
    size: usize,
    unsolved: Vec<[u8; 81]>,
    solved: Vec<[u8; 81]>,
}

struct puzzle {
    puzz: [u8; 81],

}

fn get_puzzles(file:&str) -> puzzles {
    let mut p = puzzles {
        size: 0,
        unsolved: vec![],
        solved: vec![],
    };
    let puzzles_string = fs::read_to_string(file);
    for line in puzzles_string.unwrap().lines() {
        if let Some((unsolved, solved)) = line.split_once(",") {
            p.size += 1;
            let mut unsolved_list:[u8;81] = [0;81];
            let mut solved_list: [u8;81] = [0;81];

            for num in 0..unsolved.as_bytes().len() {
                unsolved_list[num] = unsolved.as_bytes()[num] - '0' as u8;
        }
            for num in 0..solved.as_bytes().len() {
                solved_list[num] = solved.as_bytes()[num] - '0' as u8;
        }
            p.unsolved.push(unsolved_list);
            p.solved.push(solved_list);

    }
    }
    p
}


fn main() {
    let puzzles = puzzles {
        size: 0,
        unsolved: vec![],
        solved: vec![],
    };
    let puzzles = get_puzzles("puzzles.csv");
    for puzzle in puzzles.unsolved {
        println!("{:?}",puzzle);
    }
}
