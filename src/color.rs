pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Self{
        Self::new(255,0,0)
    }

    pub fn folklore_sky() -> Self {
        Self::new(72, 92, 110)  // Un gris azulado
    }

    pub fn folklore_ground() -> Self {
        Self::new(65, 54, 39)  // Un marrón oscuro
    }
}