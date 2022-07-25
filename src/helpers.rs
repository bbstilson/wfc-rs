use crate::data::{coord_2d::Coord2d, direction::Direction};

pub fn get_neighbors(
    grid_width: usize,
    grid_height: usize,
    coord: &Coord2d,
) -> Vec<(Coord2d, Direction)> {
    let x = coord.x;
    let y = coord.y;
    vec![
        // ((x - 1, y - 1), Direction::UpLeft),
        ((x, y - 1), Direction::Up),
        // ((x + 1, y - 1), Direction::UpRight),
        ((x - 1, y), Direction::Left),
        ((x + 1, y), Direction::Right),
        // ((x - 1, y + 1), Direction::DownLeft),
        ((x, y + 1), Direction::Down),
        // ((x + 1, y + 1), Direction::DownRight),
    ]
    .iter()
    .filter(|((x, y), _)| x >= &0 && y >= &0 && x < &(grid_width as i32) && y < &(grid_height as i32))
    .map(|((x, y), direction)| (Coord2d { x: *x, y: *y }, *direction))
    .collect()
}

#[cfg(test)]
mod test {
    use crate::{data::coord_2d::Coord2d, data::direction::Direction};

    use super::get_neighbors;

    #[test]
    fn test_get_neighbors() {
        let image_width = 3;
        let image_height = 3;

        let cases = vec![
            (
                Coord2d { x: 0, y: 0 },
                vec![
                    (Coord2d { x: 1, y: 0 }, Direction::Right),
                    (Coord2d { x: 0, y: 1 }, Direction::Down),
                    // (Coord2d { x: 1, y: 1 }, Direction::DownRight),
                ],
            ),
            (
                Coord2d { x: 1, y: 0 },
                vec![
                    (Coord2d { x: 0, y: 0 }, Direction::Left),
                    (Coord2d { x: 2, y: 0 }, Direction::Right),
                    // (Coord2d { x: 0, y: 1 }, Direction::DownLeft),
                    (Coord2d { x: 1, y: 1 }, Direction::Down),
                    // (Coord2d { x: 2, y: 1 }, Direction::DownRight),
                ],
            ),
            (
                Coord2d { x: 2, y: 0 },
                vec![
                    (Coord2d { x: 1, y: 0 }, Direction::Left),
                    // (Coord2d { x: 1, y: 1 }, Direction::DownLeft),
                    (Coord2d { x: 2, y: 1 }, Direction::Down),
                ],
            ),
            (
                Coord2d { x: 0, y: 1 },
                vec![
                    (Coord2d { x: 0, y: 0 }, Direction::Up),
                    // (Coord2d { x: 1, y: 0 }, Direction::UpRight),
                    (Coord2d { x: 1, y: 1 }, Direction::Right),
                    (Coord2d { x: 0, y: 2 }, Direction::Down),
                    // (Coord2d { x: 1, y: 2 }, Direction::DownRight),
                ],
            ),
            (
                Coord2d { x: 1, y: 1 },
                vec![
                    // (Coord2d { x: 0, y: 0 }, Direction::UpLeft),
                    (Coord2d { x: 1, y: 0 }, Direction::Up),
                    // (Coord2d { x: 2, y: 0 }, Direction::UpRight),
                    (Coord2d { x: 0, y: 1 }, Direction::Left),
                    (Coord2d { x: 2, y: 1 }, Direction::Right),
                    // (Coord2d { x: 0, y: 2 }, Direction::DownLeft),
                    (Coord2d { x: 1, y: 2 }, Direction::Down),
                    // (Coord2d { x: 2, y: 2 }, Direction::DownRight),
                ],
            ),
            (
                Coord2d { x: 2, y: 1 },
                vec![
                    // (Coord2d { x: 1, y: 0 }, Direction::UpLeft),
                    (Coord2d { x: 2, y: 0 }, Direction::Up),
                    (Coord2d { x: 1, y: 1 }, Direction::Left),
                    // (Coord2d { x: 1, y: 2 }, Direction::DownLeft),
                    (Coord2d { x: 2, y: 2 }, Direction::Down),
                ],
            ),
            (
                Coord2d { x: 0, y: 2 },
                vec![
                    (Coord2d { x: 0, y: 1 }, Direction::Up),
                    // (Coord2d { x: 1, y: 1 }, Direction::UpRight),
                    (Coord2d { x: 1, y: 2 }, Direction::Right),
                ],
            ),
            (
                Coord2d { x: 1, y: 2 },
                vec![
                    // (Coord2d { x: 0, y: 1 }, Direction::UpLeft),
                    (Coord2d { x: 1, y: 1 }, Direction::Up),
                    // (Coord2d { x: 2, y: 1 }, Direction::UpRight),
                    (Coord2d { x: 0, y: 2 }, Direction::Left),
                    (Coord2d { x: 2, y: 2 }, Direction::Right),
                ],
            ),
            (
                Coord2d { x: 2, y: 2 },
                vec![
                    // (Coord2d { x: 1, y: 1 }, Direction::UpLeft),
                    (Coord2d { x: 2, y: 1 }, Direction::Up),
                    (Coord2d { x: 1, y: 2 }, Direction::Left),
                ],
            ),
        ];

        for (Coord2d, expected) in cases {
            assert_eq!(get_neighbors(image_width, image_height, &Coord2d), expected)
        }
    }
}
