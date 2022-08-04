pub const BLACK: RGB = [0, 0, 0];

pub trait Color {
    fn blend(&self, other: &Self) -> Self;
}

pub type RGB = [u8; 3];

impl Color for RGB {
    fn blend(&self, other: &RGB) -> RGB {
        let mut out: RGB = [0; 3];
        for idx in 0..self.len() {
            out[idx] = (((self[idx] as u16) + (other[idx] as u16)) / 2) as u8
        }
        out
    }
}
