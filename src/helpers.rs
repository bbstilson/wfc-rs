use crate::data::{coord_2d::Vector2, direction::Direction};

pub fn get_neighbors(dimensions: (usize, usize), coord: &Vector2) -> Vec<(Vector2, Direction)> {
    let (width, height) = dimensions;
    let x = coord.x;
    let y = coord.y;
    vec![
        ((x, y - 1), Direction::Up),
        ((x - 1, y), Direction::Left),
        ((x + 1, y), Direction::Right),
        ((x, y + 1), Direction::Down),
    ]
    .iter()
    .filter(|((x, y), _)| x >= &0 && y >= &0 && x < &(width as i32) && y < &(height as i32))
    .map(|((x, y), direction)| (Vector2 { x: *x, y: *y }, *direction))
    .collect()
}

#[cfg(test)]
mod test {
    use crate::{data::coord_2d::Vector2, data::direction::Direction};

    use super::get_neighbors;

    #[test]
    fn test_get_neighbors() {
        let image_width = 3;
        let image_height = 3;

        let cases = vec![
            (
                Vector2 { x: 0, y: 0 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::Right),
                    (Vector2 { x: 0, y: 1 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 1, y: 0 },
                vec![
                    (Vector2 { x: 0, y: 0 }, Direction::Left),
                    (Vector2 { x: 2, y: 0 }, Direction::Right),
                    (Vector2 { x: 1, y: 1 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 2, y: 0 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::Left),
                    (Vector2 { x: 2, y: 1 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 0, y: 1 },
                vec![
                    (Vector2 { x: 0, y: 0 }, Direction::Up),
                    (Vector2 { x: 1, y: 1 }, Direction::Right),
                    (Vector2 { x: 0, y: 2 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 1, y: 1 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::Up),
                    (Vector2 { x: 0, y: 1 }, Direction::Left),
                    (Vector2 { x: 2, y: 1 }, Direction::Right),
                    (Vector2 { x: 1, y: 2 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 2, y: 1 },
                vec![
                    (Vector2 { x: 2, y: 0 }, Direction::Up),
                    (Vector2 { x: 1, y: 1 }, Direction::Left),
                    (Vector2 { x: 2, y: 2 }, Direction::Down),
                ],
            ),
            (
                Vector2 { x: 0, y: 2 },
                vec![
                    (Vector2 { x: 0, y: 1 }, Direction::Up),
                    (Vector2 { x: 1, y: 2 }, Direction::Right),
                ],
            ),
            (
                Vector2 { x: 1, y: 2 },
                vec![
                    (Vector2 { x: 1, y: 1 }, Direction::Up),
                    (Vector2 { x: 0, y: 2 }, Direction::Left),
                    (Vector2 { x: 2, y: 2 }, Direction::Right),
                ],
            ),
            (
                Vector2 { x: 2, y: 2 },
                vec![
                    (Vector2 { x: 2, y: 1 }, Direction::Up),
                    (Vector2 { x: 1, y: 2 }, Direction::Left),
                ],
            ),
        ];

        for (pos, expected) in cases {
            assert_eq!(get_neighbors((image_width, image_height), &pos), expected)
        }
    }
}
