use crate::data::{color::RGB, direction::Direction, mode::Mode};

use super::color::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<RGB>,
}

impl Tile {
    pub fn blend(self, other: Tile) -> Tile {
        Tile {
            width: self.width,
            height: self.height,
            pixels: self
                .pixels
                .iter()
                .zip(other.pixels)
                .map(|(a, b): (&RGB, RGB)| a.blend(&b))
                .collect(),
        }
    }

    pub fn at(&self, x: u32, y: u32) -> RGB {
        self.pixels[self.get_idx(x, y)].clone()
    }

    pub fn overlaps(&self, other: &Tile, dir: Direction, mode: Mode) -> bool {
        match mode {
            Mode::Tile => self.compare_tile(dir, other),
            Mode::Overlap => self.compare_overlap(dir, other),
        }
    }

    pub fn permute(&self) -> Vec<Tile> {
        let mut permutations = vec![self.clone()];
        permutations.push(rotate_90(self.clone()));
        permutations.push(rotate_180(self.clone()));
        permutations.push(rotate_270(self.clone()));
        permutations.push(flip_horizontal(self.clone()));
        permutations.push(flip_vertical(self.clone()));
        permutations.push(flip_vertical(rotate_90(self.clone())));
        permutations.push(flip_horizontal(rotate_90(self.clone())));
        permutations
    }

    fn get_idx(&self, x: u32, y: u32) -> usize {
        ((self.width * y) + x) as usize
    }

    fn compare_overlap(&self, dir: Direction, other: &Tile) -> bool {
        let (ys, xs, f_x, f_y) = match dir {
            // compare top-edge of 'a' to bottom-edge of 'b'
            Direction::UP => (1..self.height, 0..self.width, 0, -1),
            // compare bottom-edge of 'a' to top-edge of 'b'
            Direction::DOWN => (0..self.height - 1, 0..self.width, 0, 1),
            // compare left-edge of 'a' to right-edge of 'b'
            Direction::LEFT => (0..self.height, 1..self.width, -1, 0),
            // compare right-side of 'a' to left-side of 'b'
            Direction::RIGHT => (0..self.height, 0..self.width - 1, 1, 0),
        };

        for y in ys {
            for x in xs.clone() {
                if self.at((x as i32 + f_x) as u32, (y as i32 + f_y) as u32) != other.at(x, y) {
                    return false;
                }
            }
        }
        true
    }

    fn compare_tile(&self, dir: Direction, other: &Tile) -> bool {
        match dir {
            // compare top-edge of 'a' to bottom-edge of 'b'
            Direction::UP => {
                let top_row = self
                    .pixels
                    .iter()
                    .take(self.width as usize)
                    .collect::<Vec<_>>();
                let bottom_row = other
                    .pixels
                    .iter()
                    .skip((self.width * (self.height - 1)) as usize)
                    .collect::<Vec<_>>();

                top_row == bottom_row
            }
            // compare bottom-edge of 'a' to top-edge of 'b'
            Direction::DOWN => {
                let bottom_row = self
                    .pixels
                    .iter()
                    .skip((self.width * (self.height - 1)) as usize)
                    .collect::<Vec<_>>();
                let top_row = other
                    .pixels
                    .iter()
                    .take(self.width as usize)
                    .collect::<Vec<_>>();
                bottom_row == top_row
            }
            // compare left-edge of 'a' to right-edge of 'b'
            Direction::LEFT => {
                let left_column = self
                    .pixels
                    .chunks_exact(self.width as usize)
                    .map(|row| row[0])
                    .collect::<Vec<_>>();
                let right_column = other
                    .pixels
                    .chunks_exact(self.width as usize)
                    .map(|row| row[row.len() - 1])
                    .collect::<Vec<_>>();
                left_column == right_column
            }
            // compare right-side of 'a' to left-side of 'b'
            Direction::RIGHT => {
                let right_column = self
                    .pixels
                    .chunks_exact(self.width as usize)
                    .map(|row| row[row.len() - 1])
                    .collect::<Vec<_>>();
                let left_column = other
                    .pixels
                    .chunks_exact(self.width as usize)
                    .map(|row| row[0])
                    .collect::<Vec<_>>();
                right_column == left_column
            }
        }
    }
}

// papa bless: https://stackoverflow.com/a/8664879/6147439
fn rotate_90(tile: Tile) -> Tile {
    let chunked = tile
        .pixels
        .chunks(tile.width as usize)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<RGB>>>();

    let transposed = transpose(tile.width as usize, chunked);
    let new_pixels = transposed
        .iter()
        .flat_map(|row: &Vec<RGB>| {
            row.iter()
                .rev()
                .map(|c| c.clone())
                .collect::<Vec<RGB>>()
                .clone()
        })
        .collect();

    Tile {
        width: tile.width,
        height: tile.height,
        pixels: new_pixels,
    }
}

fn rotate_180(tile: Tile) -> Tile {
    let new_pixels = tile
        .pixels
        .chunks(tile.width as usize)
        .map(|row| row.iter().rev().map(|c| c.clone()))
        .rev()
        .flatten()
        .collect::<Vec<RGB>>();

    Tile {
        width: tile.width,
        height: tile.height,
        pixels: new_pixels,
    }
}

fn rotate_270(tile: Tile) -> Tile {
    let chunked = tile
        .pixels
        .chunks(tile.width as usize)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<RGB>>>();

    let transposed = transpose(tile.width as usize, chunked);
    let new_pixels = transposed
        .iter()
        .rev()
        .flat_map(|row: &Vec<RGB>| row.iter().map(|c| c.clone()).collect::<Vec<RGB>>().clone())
        .collect();

    Tile {
        width: tile.width,
        height: tile.height,
        pixels: new_pixels,
    }
}

fn flip_horizontal(tile: Tile) -> Tile {
    let new_pixels = tile
        .pixels
        .chunks(tile.width as usize)
        .flat_map(|row| row.iter().rev().map(|c| c.clone()))
        .collect();

    Tile {
        width: tile.width,
        height: tile.height,
        pixels: new_pixels,
    }
}
fn flip_vertical(tile: Tile) -> Tile {
    let new_pixels = tile
        .pixels
        .chunks(tile.width as usize)
        .rev()
        .flat_map(|row| row.iter().map(|c| c.clone()))
        .collect();

    Tile {
        width: tile.width,
        height: tile.height,
        pixels: new_pixels,
    }
}

fn transpose(width: usize, xs: Vec<Vec<RGB>>) -> Vec<Vec<RGB>> {
    let mut rotated = vec![vec![[0, 0, 0]; xs[0].len()]; xs.len()];
    for i in 0..(width as usize) {
        for j in 0..(width as usize) {
            rotated[i][j] = xs[j][i];
        }
    }
    rotated
}

#[cfg(test)]
mod tests {
    use crate::data::{color::BLACK, tile::*};

    const WHITE: RGB = [255, 255, 255];

    use super::Tile;

    #[test]
    fn test_rotate_270() {
        let pixels = vec![
            [0, 0, 0],
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3],
            [4, 4, 4],
            [5, 5, 5],
            [6, 6, 6],
            [7, 7, 7],
            [8, 8, 8],
        ];

        let expected = vec![
            [2, 2, 2],
            [5, 5, 5],
            [8, 8, 8],
            [1, 1, 1],
            [4, 4, 4],
            [7, 7, 7],
            [0, 0, 0],
            [3, 3, 3],
            [6, 6, 6],
        ];

        assert_eq!(
            rotate_270(Tile {
                width: 3,
                height: 3,
                pixels
            })
            .pixels,
            expected
        );
    }

    #[test]
    fn test_rotate_180() {
        let pixels = vec![
            [0, 0, 0],
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3],
            [4, 4, 4],
            [5, 5, 5],
            [6, 6, 6],
            [7, 7, 7],
            [8, 8, 8],
        ];
        let expected = vec![
            [8, 8, 8],
            [7, 7, 7],
            [6, 6, 6],
            [5, 5, 5],
            [4, 4, 4],
            [3, 3, 3],
            [2, 2, 2],
            [1, 1, 1],
            [0, 0, 0],
        ];
        assert_eq!(
            rotate_180(Tile {
                width: 3,
                height: 3,
                pixels
            })
            .pixels,
            expected
        );
    }

    #[test]
    fn test_rotate_90() {
        let pixels = vec![
            [0, 0, 0],
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3],
            [4, 4, 4],
            [5, 5, 5],
            [6, 6, 6],
            [7, 7, 7],
            [8, 8, 8],
        ];
        let expected = vec![
            [6, 6, 6],
            [3, 3, 3],
            [0, 0, 0],
            [7, 7, 7],
            [4, 4, 4],
            [1, 1, 1],
            [8, 8, 8],
            [5, 5, 5],
            [2, 2, 2],
        ];
        assert_eq!(
            rotate_90(Tile {
                width: 3,
                height: 3,
                pixels
            })
            .pixels,
            expected
        );
    }

    #[test]
    fn test_compare_tile() {
        let a = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };
        let b = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, BLACK, BLACK, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };

        assert!(a.overlaps(&b, Direction::UP, Mode::Tile));
        assert!(!a.overlaps(&b, Direction::DOWN, Mode::Tile));
        assert!(!a.overlaps(&b, Direction::LEFT, Mode::Tile));
        assert!(!a.overlaps(&b, Direction::RIGHT, Mode::Tile));
    }

    #[test]
    fn test_compare_up() {
        let a = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };
        let b = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, BLACK, BLACK, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };

        assert!(a.overlaps(&b, Direction::UP, Mode::Overlap))
    }

    #[test]
    fn test_compare_down() {
        let a = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };
        let b = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE, //
                BLACK, BLACK, BLACK,
            ],
        };

        assert!(a.overlaps(&b, Direction::DOWN, Mode::Overlap))
    }

    #[test]
    fn test_compare_left() {
        let a = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE, //
                BLACK, WHITE, WHITE,
            ],
        };
        let b = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                WHITE, BLACK, WHITE, //
                WHITE, BLACK, WHITE, //
                WHITE, BLACK, WHITE,
            ],
        };

        assert!(a.overlaps(&b, Direction::LEFT, Mode::Overlap))
    }

    #[test]
    fn test_compare_right() {
        let a = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                BLACK, WHITE, WHITE, BLACK, WHITE, WHITE, BLACK, WHITE, WHITE,
            ],
        };
        let b = Tile {
            width: 3,
            height: 3,
            pixels: vec![
                WHITE, WHITE, BLACK, WHITE, WHITE, BLACK, WHITE, WHITE, BLACK,
            ],
        };

        assert!(a.overlaps(&b, Direction::RIGHT, Mode::Overlap))
    }
}
