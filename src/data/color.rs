#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color(pub Vec<u8>);

impl Color {
    pub fn blend(&self, other: Color) -> Color {
        let blended: Vec<u8> = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(c1, c2)| (((*c1 as u16) + (*c2 as u16)) / 2) as u8)
            .collect();
        Color(blended)
    }
}
