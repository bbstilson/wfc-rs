use crate::data::{color::Color, direction::Direction};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub pixels: Vec<Vec<Color>>,
}

impl Tile {
    pub fn overlaps(&self, other: &Tile, direction: &Direction) -> bool {
        match direction {
            Direction::Up => Tile::compare_up(self, other),
            Direction::Down => Tile::compare_down(self, other),
            Direction::Left => Tile::compare_left(self, other),
            Direction::Right => Tile::compare_right(self, other),
        }
    }

    // compare top-edge of 'a' to bottom-edge of 'b'
    fn compare_up(a: &Tile, b: &Tile) -> bool {
        let a_edge = &a.pixels[0];
        let b_edge = &b.pixels[b.pixels.len() - 1];
        a_edge.eq(b_edge)
    }

    // compare bottom-edge of 'a' to top-edge of 'b'
    fn compare_down(a: &Tile, b: &Tile) -> bool {
        let a_edge = &a.pixels[a.pixels.len() - 1];
        let b_edge = &b.pixels[0];
        a_edge.eq(b_edge)
    }

    // compare left-edge of 'a' to right-edge of 'b'
    fn compare_left(a: &Tile, b: &Tile) -> bool {
        let a_edge = a
            .pixels
            .iter()
            .map(|row| row[0].clone())
            .collect::<Vec<Color>>();
        let b_edge = b
            .pixels
            .iter()
            .map(|row| row[row.len() - 1].clone())
            .collect::<Vec<Color>>();

        a_edge.eq(&b_edge)
    }

    // compare right-edge of 'a' to left-edge of 'b'
    fn compare_right(a: &Tile, b: &Tile) -> bool {
        let a_edge = a
            .pixels
            .iter()
            .map(|row| row[row.len() - 1].clone())
            .collect::<Vec<Color>>();
        let b_edge = b
            .pixels
            .iter()
            .map(|row| row[0].clone())
            .collect::<Vec<Color>>();

        a_edge.eq(&b_edge)
    }

    //     pub fn blend(&self, other: Tile) -> Tile {
    //         Tile {
    //             pixels: self
    //                 .pixels
    //                 .iter()
    //                 .zip(other.pixels.iter())
    //                 .map(|(v1, v2)| {
    //                     v1.iter()
    //                         .zip(v2.iter())
    //                         .map(|(c1, c2)| c1.blend(c2))
    //                         .collect()
    //                 })
    //                 .collect(),
    //         }
    //     }
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
    fn test_overlaps() {
        // OXO
        // OXO
        // OXO
        let a = Tile {
            pixels: vec![
                vec![black(), white(), black()],
                vec![black(), white(), black()],
                vec![black(), white(), black()],
            ],
        };
        // OXO
        // XXO
        // OXO
        let b = Tile {
            pixels: vec![
                vec![black(), white(), black()],
                vec![white(), white(), black()],
                vec![black(), white(), black()],
            ],
        };

        assert!(a.overlaps(&b, &Direction::Up));
        assert!(a.overlaps(&b, &Direction::Down));
        assert!(a.overlaps(&b, &Direction::Left));
        assert!(!a.overlaps(&b, &Direction::Right));

        assert!(b.overlaps(&a, &Direction::Up));
        assert!(b.overlaps(&a, &Direction::Down));
        assert!(!b.overlaps(&a, &Direction::Left));
        assert!(b.overlaps(&a, &Direction::Right));
    }
}
