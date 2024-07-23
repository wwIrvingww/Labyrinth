mod maze {
    pub mod reader;
    pub mod generator;
}

fn main() {
    let width = 16;
    let height = 8;

    // Generar el laberinto
    let maze = maze::generator::make_maze(width, height);

    // Imprimir el laberinto generado
    for row in maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
