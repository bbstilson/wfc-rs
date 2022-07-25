use crate::{direction::Direction, pixel::Pixel};

pub fn get_position(pos_x: i32, pos_y: i32, width: i32, bytes_per_color: i32) -> i32 {
    (pos_y * width + pos_x) * bytes_per_color
}

pub fn get_neighbors(
    image_width: i32,
    image_height: i32,
    pixel: &Pixel,
) -> Vec<(Pixel, Direction)> {
    let x = pixel.x;
    let y = pixel.y;
    vec![
        ((x - 1, y - 1), Direction::UpLeft),
        ((x, y - 1), Direction::Up),
        ((x + 1, y - 1), Direction::UpRight),
        ((x - 1, y), Direction::Left),
        ((x + 1, y), Direction::Right),
        ((x - 1, y + 1), Direction::DownLeft),
        ((x, y + 1), Direction::Down),
        ((x + 1, y + 1), Direction::DownRight),
    ]
    .iter()
    .filter(|((x, y), _)| x >= &0 && y >= &0 && x < &image_width && y < &image_height)
    .map(|((x, y), direction)| (Pixel { x: *x, y: *y }, *direction))
    .collect::<Vec<(Pixel, Direction)>>()
}

#[cfg(test)]
mod test {
    use crate::{direction::Direction, pixel::Pixel};

    use super::get_neighbors;

    #[test]
    fn test_get_neighbors() {
        let image_width = 3;
        let image_height = 3;

        let cases = vec![
            (
                Pixel { x: 0, y: 0 },
                vec![
                    (Pixel { x: 1, y: 0 }, Direction::Right),
                    (Pixel { x: 0, y: 1 }, Direction::Down),
                    (Pixel { x: 1, y: 1 }, Direction::DownRight),
                ],
            ),
            (
                Pixel { x: 1, y: 0 },
                vec![
                    (Pixel { x: 0, y: 0 }, Direction::Left),
                    (Pixel { x: 2, y: 0 }, Direction::Right),
                    (Pixel { x: 0, y: 1 }, Direction::DownLeft),
                    (Pixel { x: 1, y: 1 }, Direction::Down),
                    (Pixel { x: 2, y: 1 }, Direction::DownRight),
                ],
            ),
            (
                Pixel { x: 2, y: 0 },
                vec![
                    (Pixel { x: 1, y: 0 }, Direction::Left),
                    (Pixel { x: 1, y: 1 }, Direction::DownLeft),
                    (Pixel { x: 2, y: 1 }, Direction::Down),
                ],
            ),
            (
                Pixel { x: 0, y: 1 },
                vec![
                    (Pixel { x: 0, y: 0 }, Direction::Up),
                    (Pixel { x: 1, y: 0 }, Direction::UpRight),
                    (Pixel { x: 1, y: 1 }, Direction::Right),
                    (Pixel { x: 0, y: 2 }, Direction::Down),
                    (Pixel { x: 1, y: 2 }, Direction::DownRight),
                ],
            ),
            (
                Pixel { x: 1, y: 1 },
                vec![
                    (Pixel { x: 0, y: 0 }, Direction::UpLeft),
                    (Pixel { x: 1, y: 0 }, Direction::Up),
                    (Pixel { x: 2, y: 0 }, Direction::UpRight),
                    (Pixel { x: 0, y: 1 }, Direction::Left),
                    (Pixel { x: 2, y: 1 }, Direction::Right),
                    (Pixel { x: 0, y: 2 }, Direction::DownLeft),
                    (Pixel { x: 1, y: 2 }, Direction::Down),
                    (Pixel { x: 2, y: 2 }, Direction::DownRight),
                ],
            ),
            (
                Pixel { x: 2, y: 1 },
                vec![
                    (Pixel { x: 1, y: 0 }, Direction::UpLeft),
                    (Pixel { x: 2, y: 0 }, Direction::Up),
                    (Pixel { x: 1, y: 1 }, Direction::Left),
                    (Pixel { x: 1, y: 2 }, Direction::DownLeft),
                    (Pixel { x: 2, y: 2 }, Direction::Down),
                ],
            ),
            (
                Pixel { x: 0, y: 2 },
                vec![
                    (Pixel { x: 0, y: 1 }, Direction::Up),
                    (Pixel { x: 1, y: 1 }, Direction::UpRight),
                    (Pixel { x: 1, y: 2 }, Direction::Right),
                ],
            ),
            (
                Pixel { x: 1, y: 2 },
                vec![
                    (Pixel { x: 0, y: 1 }, Direction::UpLeft),
                    (Pixel { x: 1, y: 1 }, Direction::Up),
                    (Pixel { x: 2, y: 1 }, Direction::UpRight),
                    (Pixel { x: 0, y: 2 }, Direction::Left),
                    (Pixel { x: 2, y: 2 }, Direction::Right),
                ],
            ),
            (
                Pixel { x: 2, y: 2 },
                vec![
                    (Pixel { x: 1, y: 1 }, Direction::UpLeft),
                    (Pixel { x: 2, y: 1 }, Direction::Up),
                    (Pixel { x: 1, y: 2 }, Direction::Left),
                ],
            ),
        ];

        for (pixel, expected) in cases {
            assert_eq!(get_neighbors(image_width, image_height, &pixel), expected)
        }
    }
}
