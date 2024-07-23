mod maze {
    pub mod reader;
}

fn main() {
    let filename = "maze.txt";
    let maze = maze::reader::load_maze(filename);

    // Imprimir el laberinto cargado
    for row in maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
