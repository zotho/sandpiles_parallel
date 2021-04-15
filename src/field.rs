use std::collections::HashSet;

use crate::get::{Get, GetMut};

#[derive(Debug)]
pub struct Field<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
    pub affected_indexes: HashSet<usize>,
}

macro_rules! from_coords {
    ($self: ident, $x: expr, $y: expr) => {
        $self.width * $y + $x
    };
}

impl<T> Field<T> {
    pub fn to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

impl<T: Default + Clone> Field<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Field {
            width,
            height,
            data: vec![T::default(); width * height],
            affected_indexes: HashSet::new(),
        }
    }
}

impl Field<u32> {
    pub fn fill_affected_indexes(&mut self) {
        for (i, count) in self.data.iter().enumerate() {
            if *count > 4 {
                self.affected_indexes.insert(i);
            }
        }
    }

    pub fn wrong_slow_update(&mut self) {
        let incr = |cell: &mut u32| {*cell += 1; Some(*cell >= 4)};
        for i in self.affected_indexes.clone() {
            let (x, y) = self.to_coords(i);
            let count = self[(x, y)];
            if count >= 4 {
                self[(x, y)] -= 4;
                self.get_mut((x-1, y)).and_then(incr).and_then(|cell| {
                    if cell {
                        let index = from_coords!(self, x-1, y);
                        self.affected_indexes.insert(index);
                    }
                    Some(cell)
                });
                self.get_mut((x+1, y)).and_then(incr).and_then(|cell| {
                    if cell {
                        let index = from_coords!(self, x+1, y);
                        self.affected_indexes.insert(index);
                    }
                    Some(cell)
                });
                self.get_mut((x, y-1)).and_then(incr).and_then(|cell| {
                    if cell {
                        let index = from_coords!(self, x, y-1);
                        self.affected_indexes.insert(index);
                    }
                    Some(cell)
                });
                self.get_mut((x, y+1)).and_then(incr).and_then(|cell| {
                    if cell {
                        let index = from_coords!(self, x, y+1);
                        self.affected_indexes.insert(index);
                    }
                    Some(cell)
                });
            }
        }
    }

    pub fn put_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
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
            self[(x, y)] += 10; // Draw first pixel
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
                self[(x, y)] += 10;
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
            self[(x, y)] += 10; // Draw first pixel
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
                self[(x, y)] += 10;
            }
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Field<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[from_coords!(self, x, y)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Field<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[from_coords!(self, x, y)]
    }
}

impl<T> Get<(usize, usize)> for Field<T> {
    type Output = T;

    fn get(&self, (x, y): (usize, usize)) -> Option<&Self::Output> {
        (x < self.width).then(|| self.data.get(from_coords!(self, x, y))).flatten()
    }
}

impl<T> GetMut<(usize, usize)> for Field<T> {
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Self::Output> {
        (x < self.width).then(move || self.data.get_mut(from_coords!(self, x, y))).flatten()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_coords() {
        let field: Field<u8> = Field::new(10, 10);
        assert_eq!(field.to_coords(11), (1, 1));
        assert_eq!(field.to_coords(10), (0, 1));
        assert_eq!(field.to_coords(9), (9, 0));
    }
}