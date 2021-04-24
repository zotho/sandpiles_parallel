use rayon::slice::{ParallelSlice, ParallelSliceMut};
use rayon::iter::{
    IndexedParallelIterator, ParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator
};

mod get;

use get::{Get, GetMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Field<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
    pub old_data: Vec<T>,
}

impl<T: std::fmt::Debug> std::fmt::Display for Field<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = self
            .data
            .chunks(self.width)
            .map(|line| {
                line.iter()
                    .map(|cell| format!("{:?}", cell))
                    .fold(String::new(), |a, b| a + &b + ", ")
            })
            .fold(String::new(), |a, b| a + &b + "\n");

        f.write_str(&a)
    }
}

impl<T> Field<T> {
    pub fn to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn from_coords(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn from_coords_checked(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(self.width * y + x)
        } else {
            None
        }
    }
}

impl<T: Default + Clone> Field<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Field {
            width,
            height,
            data: vec![T::default(); width * height],
            old_data: vec![T::default(); width * height],
        }
    }
}

impl Field<u32> {
    pub fn update_parallel(&mut self) {
        #[cfg(feature = "elapsed")]
        let t1 = std::time::Instant::now();

        std::mem::swap(&mut self.data, &mut self.old_data);
        self.data.clone_from(&self.old_data);

        #[cfg(feature = "elapsed")]
        let e1 = t1.elapsed();
        #[cfg(feature = "elapsed")]
        let t2 = std::time::Instant::now();

        self.old_data
            .par_chunks(self.width)
            .zip(self.data.par_chunks_mut(self.width))
            .for_each(|(old_line, line)| {
                old_line
                    .iter()
                    .skip(1)
                    .zip(line.iter_mut())
                    .for_each(|(&old_cell, prev_cell)| {
                        *prev_cell += ((old_cell >= 4) as u32) * 1
                    });
                old_line
                    .iter()
                    .zip(line.iter_mut().skip(1))
                    .for_each(|(&old_cell, next_cell)| {
                        *next_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        #[cfg(feature = "elapsed")]
        let e2 = t2.elapsed();
        #[cfg(feature = "elapsed")]
        let t3 = std::time::Instant::now();

        self.old_data
            .par_chunks(self.width)
            .skip(1)
            .zip(self.data.par_chunks_mut(self.width))
            .for_each(|(old_line, prev_line)| {
                old_line
                    .iter()
                    .zip(prev_line.iter_mut())
                    .for_each(|(&old_cell, up_cell)| {
                        *up_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        #[cfg(feature = "elapsed")]
        let e3 = t3.elapsed();
        #[cfg(feature = "elapsed")]
        let t4 = std::time::Instant::now();

        self.old_data
            .par_chunks(self.width)
            .zip(self.data.par_chunks_mut(self.width).skip(1))
            .for_each(|(old_line, next_line)| {
                old_line
                    .iter()
                    .zip(next_line.iter_mut())
                    .for_each(|(&old_cell, down_cell)| {
                        *down_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        #[cfg(feature = "elapsed")]
        let e4 = t4.elapsed();
        #[cfg(feature = "elapsed")]
        let t5 = std::time::Instant::now();

        self.old_data
            .par_iter()
            .zip(self.data.par_iter_mut())
            .for_each(|(&old_cell, cell)| {
                *cell -= ((old_cell >= 4) as u32) * 4
            });

        #[cfg(feature = "elapsed")]
        let e5 = t5.elapsed();

        #[cfg(feature = "elapsed")]
        println!("{}\n{}\n{}\n{}\n{}\n", e1.as_secs_f64(), e2.as_secs_f64(), e3.as_secs_f64(), e4.as_secs_f64(), e5.as_secs_f64());
    }

    pub fn update_iter_branchless(&mut self) {
        std::mem::swap(&mut self.data, &mut self.old_data);
        self.data.clone_from(&self.old_data);

        self.old_data
            .chunks(self.width)
            .zip(self.data.chunks_mut(self.width))
            .for_each(|(old_line, line)| {
                old_line
                    .iter()
                    .skip(1)
                    .zip(line.iter_mut())
                    .for_each(|(&old_cell, prev_cell)| {
                        *prev_cell += ((old_cell >= 4) as u32) * 1
                    });
                old_line
                    .iter()
                    .zip(line.iter_mut().skip(1))
                    .for_each(|(&old_cell, next_cell)| {
                        *next_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        self.old_data
            .chunks(self.width)
            .skip(1)
            .zip(self.data.chunks_mut(self.width))
            .for_each(|(old_line, prev_line)| {
                old_line
                    .iter()
                    .zip(prev_line.iter_mut())
                    .for_each(|(&old_cell, up_cell)| {
                        *up_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        self.old_data
            .chunks(self.width)
            .zip(self.data.chunks_mut(self.width).skip(1))
            .for_each(|(old_line, next_line)| {
                old_line
                    .iter()
                    .zip(next_line.iter_mut())
                    .for_each(|(&old_cell, down_cell)| {
                        *down_cell += ((old_cell >= 4) as u32) * 1
                    });
            });

        self.old_data
            .iter()
            .zip(self.data.iter_mut())
            .for_each(|(&old_cell, cell)| {
                *cell -= ((old_cell >= 4) as u32) * 4
            });
    }

    pub fn update_iter(&mut self) {
        std::mem::swap(&mut self.data, &mut self.old_data);
        self.data.clone_from(&self.old_data);

        self.old_data
            .chunks(self.width)
            .zip(self.data.chunks_mut(self.width))
            .for_each(|(old_line, line)| {
                old_line
                    .iter()
                    .skip(1)
                    .zip(line.iter_mut())
                    .for_each(|(old_cell, prev_cell)| {
                        if *old_cell >= 4 {
                            *prev_cell += 1
                        }
                    });
                old_line
                    .iter()
                    .zip(line.iter_mut().skip(1))
                    .for_each(|(old_cell, next_cell)| {
                        if *old_cell >= 4 {
                            *next_cell += 1
                        }
                    });
            });

        self.old_data
            .chunks(self.width)
            .skip(1)
            .zip(self.data.chunks_mut(self.width))
            .for_each(|(old_line, prev_line)| {
                old_line
                    .iter()
                    .zip(prev_line.iter_mut())
                    .for_each(|(old_cell, up_cell)| {
                        if *old_cell >= 4 {
                            *up_cell += 1
                        }
                    });
            });

        self.old_data
            .chunks(self.width)
            .zip(self.data.chunks_mut(self.width).skip(1))
            .for_each(|(old_line, next_line)| {
                old_line
                    .iter()
                    .zip(next_line.iter_mut())
                    .for_each(|(old_cell, down_cell)| {
                        if *old_cell >= 4 {
                            *down_cell += 1
                        }
                    });
            });

        self.old_data
            .iter()
            .zip(self.data.iter_mut())
            .for_each(|(old_cell, cell)| {
                if *old_cell >= 4 {
                    *cell -= 4
                }
            });
    }

    pub fn slow_update(&mut self) {
        std::mem::swap(&mut self.data, &mut self.old_data);
        self.data.clone_from(&self.old_data);

        for i in 0..self.old_data.len() {
            if self.old_data[i] >= 4 {
                let (x, y) = self.to_coords(i);
                if x > 0 {
                    self.data[i - 1] += 1;
                }
                if x + 1 < self.width {
                    self.data[i + 1] += 1;
                }
                if y > 0 {
                    self.data[i - self.width] += 1;
                }
                if y + 1 < self.height {
                    self.data[i + self.width] += 1;
                }
                self.data[i] -= 4;
            }
        }
    }

    pub fn put_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let increment = 10;
        // Source: https://jstutorial.medium.com/how-to-code-your-first-algorithm-draw-a-line-ca121f9a1395
        // Calculate line deltas
        let dx = x2 as i16 - x1 as i16;
        let dy = y2 as i16 - y1 as i16;
        // Create a positive copy of deltas (makes iterating easier)
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        // Calculate error intervals for both axis
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;
        // The line is X-axis dominant
        if dy1 <= dx1 {
            // Line is drawn left to right
            let (x, mut y, xe) = if dx >= 0 {
                (x1, y1, x2)
            } else {
                // Line is drawn right to left (swap ends)
                (x2, y2, x1)
            };
            self.get_mut((x, y)).map(|cell| *cell += increment); // Draw first pixel
                                                                 // Rasterize the line
            for x in x..xe {
                // Deal with octants...
                if px < 0 {
                    px += 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                // Draw pixel from line span at
                // currently rasterized position
                self.get_mut((x, y)).map(|cell| *cell += increment);
            }
        } else {
            // The line is Y-axis dominant
            // Line is drawn bottom to top
            let (mut x, y, ye) = if dy >= 0 {
                (x1, y1, y2)
            } else {
                // Line is drawn top to bottom
                (x2, y2, y1)
            };
            self.get_mut((x, y)).map(|cell| *cell += increment); // Draw first pixel
                                                                 // Rasterize the line
            for y in y..ye {
                // Deal with octants...
                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }
                // Draw pixel from line span at
                // currently rasterized position
                self.get_mut((x, y)).map(|cell| *cell += increment);
            }
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Field<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[self.from_coords(x, y)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Field<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let index = self.from_coords(x, y);
        &mut self.data[index]
    }
}

impl<T> std::ops::Index<usize> for Field<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Field<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Get<usize> for Field<T> {
    type Output = T;

    fn get(&self, index: usize) -> Option<&Self::Output> {
        self.data.get(index)
    }
}

impl<T> GetMut<usize> for Field<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        self.data.get_mut(index)
    }
}

impl<T> Get<(usize, usize)> for Field<T> {
    type Output = T;

    fn get(&self, (x, y): (usize, usize)) -> Option<&Self::Output> {
        (x < self.width)
            .then(|| self.data.get(self.from_coords(x, y)))
            .flatten()
    }
}

impl<T> GetMut<(usize, usize)> for Field<T> {
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Self::Output> {
        let index = self.from_coords(x, y);
        (x < self.width)
            .then(move || self.data.get_mut(index))
            .flatten()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn test_to_coords() {
        let field: Field<u8> = Field::new(10, 10);
        assert_eq!(field.to_coords(11), (1, 1));
        assert_eq!(field.to_coords(10), (0, 1));
        assert_eq!(field.to_coords(9), (9, 0));
    }

    #[test]
    fn test_update_iter() {
        let mut rng = StdRng::seed_from_u64(345234);
        let mut field: Field<u32> = Field::new(100, 100);
        field
            .data
            .iter_mut()
            .for_each(|cell| *cell = rng.gen_range(0..=400));
        let mut field_2 = field.clone();
        let mut field_3 = field.clone();
        let mut field_4 = field.clone();
        for _ in 0..100 {
            field.slow_update();
            field_2.update_iter();
            field_3.update_iter_branchless();
            field_4.update_parallel();
        }
        assert_eq!(field, field_2);
        assert_eq!(field_2, field_3);
        assert_eq!(field_2, field_4);
    }

    // #[test]
    // fn test_update() {
    //     let mut rng = StdRng::seed_from_u64(345234);
    //     let mut field: Field<u32> = Field::new(5, 5);
    //     field.data.iter_mut().for_each(|cell| *cell = rng.gen_range(0..=5));
    //     let mut field_2 = field.clone();
    //     println!("{}\n", field);
    //     field.slow_update();
    //     field_2.update_iter();
    //     println!("{}\n", field);
    //     println!("{}", field_2);
    // }

    #[test]
    fn test_step() {
        let mut field: Field<u32> = Field::new(3, 3);
        let index = field.from_coords(1, 1);
        field.data[index] = 4;
        field.update_iter();

        let mut expected_field: Field<u32> = Field::new(3, 3);
        expected_field
            .data
            .copy_from_slice(&[0, 1, 0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(field.data, expected_field.data);
    }
}
