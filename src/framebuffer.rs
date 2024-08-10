pub struct Framebuffer { // Estructura pública (puede utilizarse en otros módulos)
    pub width: usize, 
    pub height: usize, 
    pub buffer: Vec<u32>, // Guarda un vector de valores de pixeles, donde cada elemento representa un color en formato u32 (un entero de 32 bits sin signo).
    background_color: u32, // Guarda el fondo del framebuffer
    current_color: u32, // Guarda el color actual que se usará para dibujar
}

impl Framebuffer{
    pub fn new(width: usize, height: usize) -> Self{ // Crea una nueva instancia de un framebuffer
        Self {
            width,
            height,
            buffer: vec![0; width * height], // Inicializa el buffer con ceros
            background_color: 0x000000, 
            current_color: 0xFFFFFF,    
        } // Retorna una nueva instancia del Framebuffer
    }

    pub fn clear(&mut self) { // Elimina el framebuffer, llenándolo con el color de fondo
        for pixel in self.buffer.iter_mut() { // Itera con un iterador mutable (sobre el arreglo del buffer) por cada pixel, para ponerle el color de fondo
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) { // Dibuja un punto en el framebuffer
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) { // Establece el color de fondo del framebuffer
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) { //Establece el color actual para dibujar en el framebuffer
        self.current_color = color;
    }
}