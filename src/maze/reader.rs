use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze = vec![];
    let mut player_start = (0, 0);

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        if let Some(x) = row.iter().position(|&c| c == 'p') {
            player_start = (x, y);
        }
        maze.push(row);
    }

    (maze, player_start)
}
