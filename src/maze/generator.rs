use rand::seq::SliceRandom; //Permite el uso del método shuffle para mezclar elementos en un vector.
use rand::Rng; //Proporciona funcionalidades para generar números aleatorios.

//w: ancho, h: alto
pub fn make_maze(w: usize, h: usize) -> Vec<Vec<char>> { //La función devuelve un vector de vectores de caracteres, representando el laberinto.
    //#Inicializacion de las matrices#
    let mut rng = rand::thread_rng(); //generador aleatorio

    let mut vis = vec![vec![0; w + 1]; h + 1]; //matriz 'vis'. vis es una matriz de dimensiones (h+1) x (w+1) inicializada con ceros. Se utiliza para marcar las celdas visitadas.
    
    //Los siguientes dos for sirven para marcar los bordes. Se marcan con unos para indicar que son muros.
    for i in 0..=h {
        vis[i][w] = 1;
    }
    for j in 0..=w {
        vis[h][j] = 1;
    }

    //##Matrices##
    let mut ver = vec![vec!['|'; w + 1]; h];//representa las paredes verticales del laberinto.
    for i in 0..h {
        ver[i][w] = '|';
    }

    let mut hor = vec![vec!['+'; w + 1]; h + 1];//representa las paredes horizontales del laberinto.
    for i in 0..=h {
        for j in 0..w {
            hor[i][j] = '+';
        }
    }

    //Esta función es recursiva y se encarga de recorrer el laberinto y crear caminos.
    fn walk(x: usize, y: usize, w: usize, h: usize, vis: &mut Vec<Vec<i32>>, ver: &mut Vec<Vec<char>>, hor: &mut Vec<Vec<char>>, rng: &mut rand::rngs::ThreadRng) {
        vis[y][x] = 1; //Marca la Celda actual como visitada
        //Define las posibles direcciones de movimiento.
        let mut d = vec![(x as isize - 1, y as isize), (x as isize, y as isize + 1), (x as isize + 1, y as isize), (x as isize, y as isize - 1)];
        d.shuffle(rng); //mezcla las direcciones
        //Recorrer las direcciones
        for &(xx, yy) in &d {
            if xx < 0 || yy < 0 || xx >= w as isize || yy >= h as isize || vis[yy as usize][xx as usize] != 0 {
                continue;
            }
            let xx = xx as usize;
            let yy = yy as usize;
            if xx == x {
                hor[std::cmp::max(y, yy)][x] = '+';
            }
            if yy == y {
                ver[y][std::cmp::max(x, xx)] = ' ';
            }
            walk(xx, yy, w, h, vis, ver, hor, rng);
        }
    }

    walk(rng.gen_range(0..w), rng.gen_range(0..h), w, h, &mut vis, &mut ver, &mut hor, &mut rng);

    // Combina 'hor' y 'ver' para crear la representacion del laberinto
    let mut maze = Vec::new();
    for i in 0..=h {
        let mut line = Vec::new();
        for j in 0..=w {
            line.push(hor[i][j]);
            if j < w {
                line.push(' ');
            }
        }
        maze.push(line);
        if i < h {
            let mut line = Vec::new();
            for j in 0..=w {
                line.push(ver[i][j]);
                if j < w {
                    line.push(' ');
                }
            }
            maze.push(line);
        }
    }

    // Set start and goal positions
    maze[1][1] = 'p'; //punto de inicio
    maze[h * 2 - 1][w * 2 - 1] = 'g'; //meta

    maze
}