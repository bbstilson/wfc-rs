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
        if y == 0 {
            None
        } else {
            Some((Pixel { x: x, y: y - 1 }, Direction::UP))
        },
        if y == (image_height - 1) {
            None
        } else {
            Some((Pixel { x: x, y: y + 1 }, Direction::DOWN))
        },
        if x == 0 {
            None
        } else {
            Some((Pixel { x: x - 1, y }, Direction::LEFT))
        },
        if x == (image_width - 1) {
            None
        } else {
            Some((Pixel { x: x + 1, y }, Direction::RIGHT))
        },
    ]
    .iter()
    .flatten()
    .map(|(neighbor, direction)| (*neighbor, *direction))
    .collect::<Vec<(Pixel, Direction)>>()
}

// .map(|(neighbor, direction)| (pixel_to_id[neighbor], *direction))
