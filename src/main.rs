use crate::includes::coeff_matrix::CoEffMatrix;
use crate::includes::coeff_matrix::Matrix;
use crate::includes::model::Model;
use crate::includes::coord::CoOrd;
use crate::includes::compat::Compat;
//use rand::{random, Rng};
use std::collections::{HashMap, HashSet};

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

fn main() {
    let input_matrix = Matrix(vec![
        ['L', 'L', 'L', 'L'].to_vec(),
        ['L', 'L', 'L', 'L'].to_vec(),
        ['L', 'L', 'L', 'L'].to_vec(),
        ['L', 'C', 'C', 'L'].to_vec(),
        ['C', 'S', 'S', 'C'].to_vec(),
        ['S', 'S', 'S', 'S'].to_vec(),
        ['S', 'S', 'S', 'S'].to_vec(),
    ]);

    let (compats, weights) = parse_matrix(&input_matrix);

    let mut coeff: CoEffMatrix<Vec<char>> = CoEffMatrix::new((15, 15), &weights);

    //println!("initial co-eff matrix");

   // println!("{}", coeff);

    //println!("initial entropies : ");
   // (0..3).for_each(|x| (0..3).for_each(|y| println!("{}", coeff.entropy(CoOrd { x: x, y: y }))));
    let mut model = Model::new(&mut coeff, &compats);
    model.run();
    println!("{}", model.coeff);
}
