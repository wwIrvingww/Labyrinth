use std::fs::File;
use std::io::{BufRead, BufReader};

//La función toma un parámetro, filename, que es una referencia a una cadena de caracteres 
//La función devuelve un Vec<Vec<char>>, que es un vector de vectores de caracteres.
pub fn load_maze(filename: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut player_pos = (0, 0);

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row_vec = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            if ch == 'p' {
                player_pos = (row, col);
            }
            row_vec.push(ch);
        }
        maze.push(row_vec);
    }

    (maze, player_pos)
}