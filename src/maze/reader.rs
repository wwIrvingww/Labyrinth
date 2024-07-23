use std::fs::File;
use std::io::{BufRead, BufReader};

//La función toma un parámetro, filename, que es una referencia a una cadena de caracteres 
//La función devuelve un Vec<Vec<char>>, que es un vector de vectores de caracteres.
pub fn load_maze(filename: &str) -> Vec<Vec<char>> {

    let file = File::open(filename).unwrap();  //Abre el archivo con el nombre proporcionado. Unwrap es manejo de errores
    let reader = BufReader::new(file); //Envuelve el archivo en un BufReader, que proporciona un buffer para leer el archivo

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}