use std::fs;


struct Puzzles {
    size: usize,
    unsolved: Vec<[u8; 81]>,
    solved: Vec<[u8; 81]>,
}

struct Puzzle {
    puzz: [u8; 81],

}

fn get_puzzles(file:&str) -> Puzzles {
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
        
        if let Some((unsolved, solved)) = line.split_once(",") {
            p.size += 1;
            let mut unsolved_list:[u8;81] = [0;81];
            let mut solved_list: [u8;81] = [0;81];
            let unsolved_bytes = unsolved.as_bytes();
            let solved_bytes = solved.as_bytes();

            for num in 0..81 {
                unsolved_list[num] = unsolved_bytes[num] - '0' as u8;
                solved_list[num] = solved_bytes[num] - '0' as u8;
            }
            p.unsolved.push(unsolved_list);
            p.solved.push(solved_list);
        }
    }
    p
}


fn main() {
    let now = std::time::Instant::now();
    let puzzles = get_puzzles("Puzzles.csv");
    let elapsed = now.elapsed();
    for puzzle in puzzles.unsolved {
        println!("{:?}",puzzle);
    }
    println!("Loaded {} Puzzles in {:?}", puzzles.size, elapsed);
}
