pub fn get_position(pos_x: i32, pos_y: i32, width: i32, bytes_per_color: i32) -> i32 {
    (pos_y * width + pos_x) * bytes_per_color
}
