use crate::data::{direction::Direction, vector2::Vector2};

pub fn get_neighbors(dimensions: (u32, u32), coord: &Vector2) -> Vec<(Vector2, Direction)> {
    let (width, height) = dimensions;
    let x = coord.x;
    let y = coord.y;
    vec![
        ((x, y - 1), Direction::UP),
        ((x, y + 1), Direction::DOWN),
        ((x - 1, y), Direction::LEFT),
        ((x + 1, y), Direction::RIGHT),
    ]
    .iter()
    .filter(|((x, y), _)| x >= &0 && y >= &0 && x < &(width as i32) && y < &(height as i32))
    .map(|((x, y), direction)| (Vector2 { x: *x, y: *y }, *direction))
    .collect()
}

#[cfg(test)]
mod test {
    use crate::data::{direction::Direction, vector2::Vector2};

    use super::get_neighbors;

    #[test]
    fn test_get_neighbors() {
        let image_width = 3;
        let image_height = 3;

        let cases = vec![
            (
                Vector2 { x: 0, y: 0 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::RIGHT),
                    (Vector2 { x: 0, y: 1 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 1, y: 0 },
                vec![
                    (Vector2 { x: 0, y: 0 }, Direction::LEFT),
                    (Vector2 { x: 2, y: 0 }, Direction::RIGHT),
                    (Vector2 { x: 1, y: 1 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 2, y: 0 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::LEFT),
                    (Vector2 { x: 2, y: 1 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 0, y: 1 },
                vec![
                    (Vector2 { x: 0, y: 0 }, Direction::UP),
                    (Vector2 { x: 1, y: 1 }, Direction::RIGHT),
                    (Vector2 { x: 0, y: 2 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 1, y: 1 },
                vec![
                    (Vector2 { x: 1, y: 0 }, Direction::UP),
                    (Vector2 { x: 0, y: 1 }, Direction::LEFT),
                    (Vector2 { x: 2, y: 1 }, Direction::RIGHT),
                    (Vector2 { x: 1, y: 2 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 2, y: 1 },
                vec![
                    (Vector2 { x: 2, y: 0 }, Direction::UP),
                    (Vector2 { x: 1, y: 1 }, Direction::LEFT),
                    (Vector2 { x: 2, y: 2 }, Direction::DOWN),
                ],
            ),
            (
                Vector2 { x: 0, y: 2 },
                vec![
                    (Vector2 { x: 0, y: 1 }, Direction::UP),
                    (Vector2 { x: 1, y: 2 }, Direction::RIGHT),
                ],
            ),
            (
                Vector2 { x: 1, y: 2 },
                vec![
                    (Vector2 { x: 1, y: 1 }, Direction::UP),
                    (Vector2 { x: 0, y: 2 }, Direction::LEFT),
                    (Vector2 { x: 2, y: 2 }, Direction::RIGHT),
                ],
            ),
            (
                Vector2 { x: 2, y: 2 },
                vec![
                    (Vector2 { x: 2, y: 1 }, Direction::UP),
                    (Vector2 { x: 1, y: 2 }, Direction::LEFT),
                ],
            ),
        ];

        for (pos, expected) in cases {
            assert_eq!(get_neighbors((image_width, image_height), &pos), expected)
        }
    }
}
