use std::vec;

use crate::data::{color::RGB, direction, direction::Direction};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub pixels: Vec<Vec<RGB>>,
}

impl Tile {
    pub fn at(&self, x: usize, y: usize) -> RGB {
        self.pixels[y][x].clone()
    }

    pub fn overlaps(&self, other: &Tile, dir: &Direction, tile_dimensions: (usize, usize)) -> bool {
        match dir {
            &direction::UP => Tile::compare_up(self, other, tile_dimensions),
            &direction::DOWN => Tile::compare_down(self, other, tile_dimensions),
            &direction::LEFT => Tile::compare_left(self, other, tile_dimensions),
            &direction::RIGHT => Tile::compare_right(self, other, tile_dimensions),
            _ => unreachable!("invalid direction"),
        }
    }

    pub fn permute(pixels: Vec<Vec<RGB>>) -> Vec<Tile> {
        let base = Tile {
            pixels: pixels.clone(),
        };
        let mut permutations = vec![];

        permutations.push(base.clone());
        // permutations.push(rotate_90(&pixels));
        // permutations.push(rotate_180(&buf))
        // permutations.push(rotate_270(&buf))
        // permutations.push(flip_horizontal(&buf))
        // permutations.push(flip_vertical(&buf))
        // permutations.push(flip_vertical(&rotate_90(&buf)))
        // permutations.push(flip_horizontal(&rotate_90(&buf)))

        permutations
    }

    // compare top-edge of 'a' to bottom-edge of 'b'
    fn compare_up(a: &Tile, b: &Tile, tile_dimensions: (usize, usize)) -> bool {
        let (width, height) = tile_dimensions;
        for y in 1..height {
            for x in 0..width {
                if a.at(x, y - 1) != b.at(x, y) {
                    return false;
                }
            }
        }
        true
    }

    // compare bottom-edge of 'a' to top-edge of 'b'
    fn compare_down(a: &Tile, b: &Tile, tile_dimensions: (usize, usize)) -> bool {
        let (width, height) = tile_dimensions;
        for y in 0..height - 1 {
            for x in 0..width {
                if a.at(x, y + 1) != b.at(x, y) {
                    return false;
                }
            }
        }
        true
    }

    // compare left-edge of 'a' to right-edge of 'b'
    fn compare_left(a: &Tile, b: &Tile, tile_dimensions: (usize, usize)) -> bool {
        let (width, height) = tile_dimensions;
        for y in 0..height {
            for x in 1..width {
                if a.at(x - 1, y) != b.at(x, y) {
                    return false;
                }
            }
        }
        true
    }

    // compare right-side of 'a' to left-side of 'b'
    fn compare_right(a: &Tile, b: &Tile, tile_dimensions: (usize, usize)) -> bool {
        let (width, height) = tile_dimensions;
        for y in 0..height {
            for x in 0..width - 1 {
                if a.at(x + 1, y) != b.at(x, y) {
                    return false;
                }
            }
        }
        true
    }
}

// papa bless: https://stackoverflow.com/a/2800033/6147439
// fn rotate_90(pixels: &Vec<Vec<RGB>>) -> Tile {
//     let m = pixels.len();
//     let n = pixels[0].len();
//     let mut out = vec![vec![BLACK; n]; m];
//     for r in 0..m {
//         for c in 0..n {
//             out[c][m - 1 - r] = pixels[r][c];
//         }
//     }
//     Tile { pixels: out }
// }

// fn rotate_180(pixels: &Vec<Vec<RGB>>) -> Tile {
//     let rows = pixels.len();
//     let cols = pixels[0].len();
//     let mut out = vec![vec![BLACK; cols]; rows];

//     // for (int i = N - 1; i >= 0; i--) {
//     //     for (int j = N - 1; j >= 0; j--)
//     //         printf("%d ", mat[i][j]);

//     for row in (0..rows).rev() {
//         for col in (0..cols).rev() {
//             out[rows - row][cols - col] = pixels[row][col];
//         }
//     }
//     Tile { pixels: out }
// }

#[cfg(test)]
mod tests {
    use crate::data::{color::BLACK, direction, tile::*};

    const WHITE: RGB = [255, 255, 255];

    use super::Tile;

    // #[test]
    // fn test_rotate_180() {
    //     let pixels = vec![
    //         vec![[0, 0, 0], [1, 1, 1], [2, 2, 2]],
    //         vec![[3, 3, 3], [4, 4, 4], [5, 5, 5]],
    //         vec![[6, 6, 6], [7, 7, 7], [8, 8, 8]],
    //     ];
    //     let expected = vec![
    //         vec![[8, 8, 8], [7, 7, 7], [6, 6, 6]],
    //         vec![[5, 5, 5], [4, 4, 4], [3, 3, 3]],
    //         vec![[2, 2, 2], [1, 1, 1], [0, 0, 0]],
    //     ];
    //     assert_eq!(rotate_180(&pixels).pixels, expected);
    // }

    // #[test]
    // fn test_rotate_90() {
    //     let pixels = vec![
    //         vec![[0, 0, 0], [1, 1, 1], [2, 2, 2]],
    //         vec![[3, 3, 3], [4, 4, 4], [5, 5, 5]],
    //         vec![[6, 6, 6], [7, 7, 7], [8, 8, 8]],
    //     ];
    //     let expected = vec![
    //         vec![[6, 6, 6], [3, 3, 3], [0, 0, 0]],
    //         vec![[7, 7, 7], [4, 4, 4], [1, 1, 1]],
    //         vec![[8, 8, 8], [5, 5, 5], [2, 2, 2]],
    //     ];
    //     assert_eq!(rotate_90(&pixels).pixels, expected);
    // }

    #[test]
    fn test_compare_up() {
        let a = Tile {
            pixels: vec![
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![BLACK, BLACK, BLACK],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
            ],
        };

        assert!(a.overlaps(&b, &direction::UP, (3, 3)))
    }

    #[test]
    fn test_compare_down() {
        let a = Tile {
            pixels: vec![
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, BLACK, BLACK],
            ],
        };

        assert!(a.overlaps(&b, &direction::DOWN, (3, 3)))
    }

    #[test]
    fn test_compare_left() {
        let a = Tile {
            pixels: vec![
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![WHITE, BLACK, WHITE],
                vec![WHITE, BLACK, WHITE],
                vec![WHITE, BLACK, WHITE],
            ],
        };

        assert!(a.overlaps(&b, &direction::LEFT, (3, 3)))
    }

    #[test]
    fn test_compare_right() {
        let a = Tile {
            pixels: vec![
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
                vec![BLACK, WHITE, WHITE],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![WHITE, WHITE, BLACK],
                vec![WHITE, WHITE, BLACK],
                vec![WHITE, WHITE, BLACK],
            ],
        };

        assert!(a.overlaps(&b, &direction::RIGHT, (3, 3)))
    }
}
