use crate::includes::coeff_matrix::CoEffMatrix;
use crate::includes::coeff_matrix::Matrix;
use crate::includes::compat::Compat;
use crate::includes::coord::CoOrd;
use crate::includes::model::Model;
use crate::includes::errors::MatrixError;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub mod includes;

fn valid_dirs(cur_co_ords: &CoOrd, matrix_size: (i32, i32)) -> Vec<CoOrd> {
    let mut dirs: Vec<CoOrd> = Vec::new();

    if cur_co_ords.y < matrix_size.1 - 1 {
        dirs.push(CoOrd::UP)
    }

    if cur_co_ords.y > 0 {
        dirs.push(CoOrd::DOWN)
    }

    if cur_co_ords.x > 0 {
        dirs.push(CoOrd::LEFT)
    }

    if cur_co_ords.x < matrix_size.0 - 1 {
        dirs.push(CoOrd::RIGHT);
    }

    return dirs;
}

fn read_matrix<P>(filename: P) -> Result<Matrix, MatrixError>
where
    P: AsRef<Path>,
{
    let mut input_matrix_vec = vec![];

    let file = File::open(filename).map_err(MatrixError::Io)?;
    let mut cols: usize;

    for line in io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .enumerate()
    {
        input_matrix_vec.push(vec![]);
        cols = line.1.len();
        for elt in line.1.chars() {
            if let Some(vec) = input_matrix_vec.last_mut() {
                vec.push(elt)
            };
        }
        if line.0 == 0 {
            continue;
        };
        if input_matrix_vec[line.0 - 1].len() != cols {
            return Err(MatrixError::ParseError);
        };
    }
    return Ok(Matrix(input_matrix_vec));
}

fn parse_matrix(matrix: &Matrix) -> (HashSet<Compat>, HashMap<char, i32>) {
    let mut compats: HashSet<Compat> = HashSet::new();
    let matrix_height = matrix.0.len();
    let matrix_width = matrix.0[0].len();
    let mut weights: HashMap<char, i32> = HashMap::new();
    for (y, row) in matrix.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            weights
                .entry(*tile)
                .and_modify(|counter| *counter += 1)
                .or_insert(0);

            for dir in valid_dirs(
                &CoOrd {
                    x: x as i32,
                    y: y as i32,
                },
                (matrix_width as i32, matrix_height as i32),
            ) {
                compats.insert(Compat {
                    tile1: *tile,
                    tile2: matrix.0[(y as i32 + dir.y) as usize][(x as i32 + dir.x) as usize],
                    direction: dir,
                });
            }
        }
    }

    (compats, weights)
}

fn main() -> Result<(), MatrixError> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Err(MatrixError::ArgsError);
    }
    let filename = &args[1];
    let (compats, weights) = parse_matrix(&read_matrix(filename)?);
    let mut coeff: CoEffMatrix<Vec<char>> = CoEffMatrix::new((15, 15), &weights);
    let mut model = Model::new(&mut coeff, &compats);
    model.run();
    println!("{}", model.coeff);
    Ok(())
}
