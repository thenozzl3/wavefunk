use crate::includes::coord::CoOrd;
use core::fmt;
use rand::Rng;
use std::collections::HashMap;

pub struct Matrix(pub Vec<Vec<char>>);

pub struct CoEffMatrix<'t, T> {
    pub width: usize,
    pub height: usize,
    pub coeff_matrix: Vec<T>,
    pub weights: &'t HashMap<char, i32>,
}

impl<'t> CoEffMatrix<'t, Vec<char>> {
    /*fill a new coeff matrix with all the possible tiles in every position */
    pub fn new(size: (usize, usize), weights: &'t HashMap<char, i32>) -> Self {
        Self {
            width: size.0,
            height: size.1,
            coeff_matrix: vec![
                weights.keys().map(|item| *item).collect();
                (size.0 * size.1) as usize
            ],
            weights,
        }
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> &mut Vec<char> {
        &mut self.coeff_matrix[x * self.width + y]
    }

    pub fn get(&self, y: usize, x: usize) -> &Vec<char> {
        &self.coeff_matrix[x * self.width + y]
    }

    pub fn set(&mut self, y: usize, x: usize, value: Vec<char>) {
        self.coeff_matrix[x * self.width + y] = value;
    }

    pub fn get_all_collapsed(&self) -> Vec<char> {
        self.coeff_matrix
            .iter()
            .filter(|tile_set| tile_set.len() == 1)
            .map(|tile| tile[0])
            .collect()
    }
    // fix this to  just use x, y...
    pub fn entropy(&self, coords: CoOrd) -> f32 {
        let mut sum_of_weights: f32 = 0.0;
        let mut sum_of_log_weights: f32 = 0.0;

        // ppbly a better way to do this with combinators .. ?
        self.get(coords.y as usize, coords.x as usize)
            .iter()
            .for_each(|tile_key| {
                sum_of_weights += self.weights.get(tile_key).unwrap().to_owned() as f32;
                sum_of_log_weights += *(self.weights.get(tile_key).unwrap()) as f32
                    * ((*(self.weights.get(tile_key).unwrap())) as f32).log2();
            });
        return (sum_of_weights as f32).log2() - (sum_of_log_weights / sum_of_weights);
    }

    pub fn all_collapsed(&self) -> bool {
        self.coeff_matrix.iter().all(|item| item.len() == 1)
    }

    pub fn collapse(&mut self, coords: CoOrd) {
        let opts = self.get(coords.y as usize, coords.x as usize);
        let filtered_tiles_with_weights: Vec<(char, i32)> = self
            .weights
            .iter()
            .map(|item| (*item.0, *item.1))
            .filter(|stuff| opts.contains(&stuff.0))
            .collect();

        let total_weights = filtered_tiles_with_weights
            .iter()
            .map(|val| val.1)
            .fold(0, |acc, e| acc + e);
        let mut rng = rand::thread_rng();
        let mut rnd: f32 = rng.gen::<f32>() * total_weights as f32;
        let mut chosen = filtered_tiles_with_weights[0].0.clone();
        for (tile, ent) in filtered_tiles_with_weights.iter() {
            rnd -= *ent as f32;
            if rnd < 0.0 {
                chosen = *tile;
                break;
            }
        }
        //println!("chosen tile: {}", chosen);
        self.set(coords.y as usize, coords.x as usize, vec![chosen]);
    }

    pub fn constrain(&mut self, coord: CoOrd, tile: &char) {
        let tile_set = self.get_mut(coord.y as usize, coord.x as usize);
        tile_set.remove(
            tile_set
                .iter()
                .position(|possible_tile| possible_tile == tile)
                .expect("not found"),
        );
    }
}

impl fmt::Display for CoEffMatrix<'_, Vec<char>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.coeff_matrix.iter().enumerate().for_each(|item| {
            // if item size is 1, then we have a collapsed cell
            if item.1.len() == 1 {
                let color_esc_start = match item.1[0] {
                    'S' => "\x1b[94m",
                    'C' => "\x1b[93m",
                    'L' => "\x1b[32m",
                    _ => "",
                };
                write!(f, "{}{}\x1b[0m ", color_esc_start, item.1[0]).unwrap();
            } else {
                write!(f, "{:?} ", item.1).unwrap();
            }
            if ((item.0 + 1) % self.width) == 0 {
                write!(f, "\n").unwrap();
            }
        });
        Ok(())
    }
}
