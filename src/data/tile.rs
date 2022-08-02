use crate::data::{color::Color, direction::Direction};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub pixels: Vec<Vec<Color>>,
}

impl Tile {
    pub fn at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x].clone()
    }

    pub fn overlaps(
        &self,
        other: &Tile,
        direction: &Direction,
        tile_dimensions: (usize, usize),
    ) -> bool {
        match direction {
            Direction::Up => Tile::compare_up(self, other, tile_dimensions),
            Direction::Down => Tile::compare_down(self, other, tile_dimensions),
            Direction::Left => Tile::compare_left(self, other, tile_dimensions),
            Direction::Right => Tile::compare_right(self, other, tile_dimensions),
        }
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

#[cfg(test)]
mod tests {
    use crate::data::{color::Color, direction::Direction};

    use super::Tile;

    fn black() -> Color {
        Color(vec![0, 0, 0])
    }
    fn white() -> Color {
        Color(vec![255, 255, 255])
    }

    #[test]
    fn test_compare_up() {
        let a = Tile {
            pixels: vec![
                vec![black(), white(), white()],
                vec![black(), white(), white()],
                vec![black(), white(), white()],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![black(), black(), black()],
                vec![black(), white(), white()],
                vec![black(), white(), white()],
            ],
        };

        assert!(a.overlaps(&b, &Direction::Up, (3, 3)))
    }

    #[test]
    fn test_compare_down() {
        let a = Tile {
            pixels: vec![
                vec![black(), white(), white()],
                vec![black(), white(), white()],
                vec![black(), white(), white()],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![black(), white(), white()],
                vec![black(), white(), white()],
                vec![black(), black(), black()],
            ],
        };

        assert!(a.overlaps(&b, &Direction::Down, (3, 3)))
    }

    #[test]
    fn test_compare_left() {
        let a = Tile {
            pixels: vec![
                vec![black(), white(), white()],
                vec![black(), white(), white()],
                vec![black(), white(), white()],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![white(), black(), white()],
                vec![white(), black(), white()],
                vec![white(), black(), white()],
            ],
        };

        assert!(a.overlaps(&b, &Direction::Left, (3, 3)))
    }

    #[test]
    fn test_compare_right() {
        let a = Tile {
            pixels: vec![
                vec![black(), white(), white()],
                vec![black(), white(), white()],
                vec![black(), white(), white()],
            ],
        };
        let b = Tile {
            pixels: vec![
                vec![white(), white(), black()],
                vec![white(), white(), black()],
                vec![white(), white(), black()],
            ],
        };

        assert!(a.overlaps(&b, &Direction::Right, (3, 3)))
    }
}
