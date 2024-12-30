use crate::CoEffMatrix;
use crate::includes::compat::Compat;
use crate::includes::coord::CoOrd;
use rand::random;

use std::collections::HashSet;


pub struct Model<'t> {
    pub coeff: &'t mut CoEffMatrix<'t,Vec<char>>,
    pub compats: &'t HashSet<Compat>,
}

impl<'a> Model<'a> {
   pub fn new(coeff_matrix: &'a mut CoEffMatrix<'a,Vec<char>>, compats: &'a HashSet<Compat>) -> Model<'a> {
        Model {
            coeff: coeff_matrix,
            compats: compats,
        }
    }
    //returns a vec of chars .. not a vec of vec of chars
    pub fn run(&mut self) -> Vec<char> {
        while !self.coeff.all_collapsed() {
            self.iterate()
        }
        self.coeff.get_all_collapsed()
    }

    pub fn iterate(&mut self) {
        let coords: CoOrd = self.find_min_entropy_coords();
        self.coeff.collapse(coords);
        self.propagate(coords);
    }

    pub fn propagate(&mut self, coord: CoOrd) {
        let mut stack: Vec<CoOrd> = vec![];
        stack.push(coord);
        let mut cur_possible_tiles: Vec<char>;

        while let Some(cur_coords) = stack.pop() {
            for dir in crate::valid_dirs(
                &cur_coords,
                (self.coeff.width as i32, self.coeff.height as i32),
            )
            .iter()
            {
                let other_co_ords: CoOrd = CoOrd {
                    x: cur_coords.x + dir.x,
                    y: cur_coords.y + dir.y,
                };

                cur_possible_tiles = self
                    .coeff
                    .get(cur_coords.y as usize, cur_coords.x as usize).clone();

                for other_tile in self
                    .coeff
                    .get(other_co_ords.y as usize, other_co_ords.x as usize).clone()
                {
                    if !cur_possible_tiles.iter().any(|cur_tile| {
                        self.compats.contains(&Compat {
                            tile2: *cur_tile,
                            tile1: other_tile,
                            direction: *dir,
                        })
                    }) {
                        self.coeff.constrain(other_co_ords, &other_tile);
                        stack.push(other_co_ords);
                    }
                }
            }
        }
    }

    pub fn find_min_entropy_coords(&self) -> CoOrd {
        let mut min_entropy = 0.0;
        let mut coord: CoOrd = CoOrd { x: 0, y: 0 };
        for y in 0..self.coeff.width {
            for x in 0..self.coeff.height {
                if self.coeff.get(y as usize, x as usize).len() == 1 {
                    continue;
                }
                let entropy_plus_noise = self.coeff.entropy(CoOrd {
                    x: y as i32,
                    y: x as i32,
                }) - (random::<f32>() / 1000 as f32);
                //println!("ent and noise {} x {} y {}", entropy_plus_noise, x, y);
                if min_entropy == 0.0 || entropy_plus_noise < min_entropy {
                    min_entropy = entropy_plus_noise;
                    coord.x = x as i32;
                    coord.y = y as i32;
                }
            }
        }
        coord
    }
}
