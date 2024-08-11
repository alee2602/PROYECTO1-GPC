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

    pub fn ground() -> Self {
        Self::new(159, 175, 200)  
    }

    pub fn black() -> Self{
        Self::new(0, 0, 0)
    }

    pub fn gradient_sky(ratio: f32) -> Self {
        let start_color = Color::new(0, 0, 0); 
        let end_color = Color::new(32, 30, 67); 

        Self::new(
            (start_color.r as f32 * (1.0 - ratio) + end_color.r as f32 * ratio) as u8,
            (start_color.g as f32 * (1.0 - ratio) + end_color.g as f32 * ratio) as u8,
            (start_color.b as f32 * (1.0 - ratio) + end_color.b as f32 * ratio) as u8,
        )
    }
}
