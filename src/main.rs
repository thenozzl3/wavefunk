use core::fmt;
use rand::{random, Rng};
use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct CoOrd {
    x: i32,
    y: i32,
}

impl CoOrd {
    pub const UP: CoOrd = CoOrd { y: 1, x: 0 };
    pub const DOWN: CoOrd = CoOrd { y: -1, x: 0 };
    pub const LEFT: CoOrd = CoOrd { y: 0, x: -1 };
    pub const RIGHT: CoOrd = CoOrd { y: 0, x: 1 };
}

struct Matrix(Vec<Vec<char>>);

struct CoEffMatrix<'t, T> {
    width: usize,
    height: usize,
    coeff_matrix: Vec<T>,
    weights: &'t HashMap<char, i32>,
}

impl<'t> CoEffMatrix<'t, Vec<char>> {
    /*fill a new coeff matrix with all the possible tiles in every position */
    fn new(size: (usize, usize), weights: &'t HashMap<char, i32>) -> Self {
        Self {
            width: size.0,
            height: size.1,
            coeff_matrix: vec![weights
                .keys()
                .map(|item| *item)
                .collect(); (size.0 * size.1) as usize],
            weights: weights,
        }
    }

    fn get_mut(&mut self, y: usize, x: usize) -> &mut Vec<char> {
        &mut self.coeff_matrix[x * self.width + y]
    }

    fn get(&self, y: usize, x: usize) -> &Vec<char> {
        &self.coeff_matrix[x * self.width + y]
    }

    fn set(&mut self, y: usize, x: usize, value: Vec<char>) {
        self.coeff_matrix[x * self.width + y] = value;
    }

    fn get_all_collapsed(&self) -> Vec<char> {
        self.coeff_matrix
            .iter()
            .filter(|tile_set| tile_set.len() == 1)
            .map(|tile| tile[0])
            .collect()
    }
    // fix this to  just use x, y...
    fn entropy(&self, coords: CoOrd) -> f32 {
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

    fn all_collapsed(&self) -> bool {
        self.coeff_matrix.iter().all(|item| item.len() == 1)
    }

    fn collapse(&mut self, coords: CoOrd) {
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

    fn constrain(&mut self, coord: CoOrd, tile: &char) {
        let tile_set = self.get_mut(coord.y as usize, coord.x as usize);
        tile_set.remove(
            tile_set
                .iter()
                .position(|possible_tile| possible_tile == tile)
                .expect("not found"),
        );
    }
}

struct Model<'t> {
    coeff: &'t mut CoEffMatrix<'t,Vec<char>>,
    compats: &'t HashSet<Compat>,
}

impl<'a> Model<'a> {
    fn new(coeff_matrix: &'a mut CoEffMatrix<'a,Vec<char>>, compats: &'a HashSet<Compat>) -> Model<'a> {
        Model {
            coeff: coeff_matrix,
            compats: compats,
        }
    }
    //returns a vec of chars .. not a vec of vec of chars
    fn run(&mut self) -> Vec<char> {
        while !self.coeff.all_collapsed() {
            self.iterate()
        }
        self.coeff.get_all_collapsed()
    }

    fn iterate(&mut self) {
        let coords: CoOrd = self.find_min_entropy_coords();
        self.coeff.collapse(coords);
        self.propagate(coords);
    }

    fn propagate(&mut self, coord: CoOrd) {
        let mut stack: Vec<CoOrd> = vec![];
        stack.push(coord);
        let mut cur_possible_tiles: Vec<char>;

        while let Some(cur_coords) = stack.pop() {
            for dir in valid_dirs(
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

    fn find_min_entropy_coords(&self) -> CoOrd {
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Compat {
    tile1: char,
    tile2: char,
    direction: CoOrd,
}

impl fmt::Display for CoOrd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x : {} y: {}", self.x, self.y)
    }
}

impl fmt::Display for Compat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.tile1, self.tile2, self.direction.x, self.direction.y
        )
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
