pub enum ColorType {
    RGB,
    RGBA,
}

impl ColorType {
    pub fn bytes_per_color(&self) -> i32 {
        match self {
            ColorType::RGB => 3,
            ColorType::RGBA => 4,
        }
    }
}
