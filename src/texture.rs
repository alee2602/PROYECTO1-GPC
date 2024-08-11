use image::GenericImageView;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let pixels = img.to_rgba8().into_raw();

        Texture {
            width: width as usize,
            height: height as usize,
            pixels,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        let index = (y * self.width + x) * 4;
        let r = self.pixels[index] as u32;
        let g = self.pixels[index + 1] as u32;
        let b = self.pixels[index + 2] as u32;
        (r << 16) | (g << 8) | b
    }
}