use std::fs::File;
use std::io::{Write, BufWriter};

use crate::framebuffer::Framebuffer;

// Constantes para el archivo BMP
const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

// Función principal para escribir un archivo BMP
pub fn write_bmp_file(file_path: &str, buffer: &[u32], width: usize, height: usize) -> std::io::Result<()> {

    // Crear un archivo y envolverlo en BufWriter para escritura eficiente
    let mut file = BufWriter::new(File::create(file_path)?);

     // Escribir el encabezado BMP en el archivo
    write_bmp_header(&mut file, width, height)?;
     // Escribir los datos de los píxeles en el archivo
    write_pixel_data(&mut file, buffer, width, height)?;

    Ok(())
}

// Función para escribir el encabezado BMP
fn write_bmp_header(file: &mut BufWriter<File>, width: usize, height: usize) -> std::io::Result<()> {

    // Calcular el tamaño del archivo y el tamaño de los píxeles
    let file_size = (height * width * (BMP_BITS_PER_PIXEL / 8)) + BMP_HEADER_SIZE as usize;
    let pixel_size = file_size - BMP_HEADER_SIZE;

    // file header
    file.write_all(b"BM")?; // Encabezado del archivo BMP
    file.write_all(&(file_size as u32).to_le_bytes())?; // Tamaño del archivo en bytes
    file.write_all(&0u32.to_le_bytes())?; // Reservado, debe ser 0
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?; // Offset de datos de píxeles

    // Información del encabezado
    file.write_all(&40u32.to_le_bytes())?; // Tamaño del encabezado de información
    file.write_all(&(width as u32).to_le_bytes())?; // Ancho de la imagen
    file.write_all(&(height as u32).to_le_bytes())?; // Altura de la imagen
    file.write_all(&1u16.to_le_bytes())?; // Número de planos de color
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?; // Bits por píxel
    file.write_all(&0u32.to_le_bytes())?; // Compresión (sin compresión)
    file.write_all(&(pixel_size as u32).to_le_bytes())?; // Tamaño de los datos de imagen
    file.write_all(&0u32.to_le_bytes())?; // Resolución horizontal (pixeles por metro, ignorado)
    file.write_all(&0u32.to_le_bytes())?; // Resolución vertical (pixeles por metro, ignorado)
    file.write_all(&0u32.to_le_bytes())?; // Número de colores en la paleta (0 para colores completos)
    file.write_all(&0u32.to_le_bytes())?; // Colores importantes (0 para todos)

    Ok(())
}

// Función para escribir los datos de los píxeles en el archivo BMP
fn write_pixel_data(file: &mut BufWriter<File>, buffer: &[u32], width: usize, height: usize) -> std::io::Result<()> {
    // Calcular el tamaño del padding para cada fila (BMP requiere que cada fila sea múltiplo de 4 bytes)
    let padding_size = (4 - (width * BMP_BITS_PER_PIXEL / 8) % 4) % 4;
    let padding = [0u8, 3];

     // Escribir los datos de los píxeles fila por fila
    for y in (0..height).rev() { // Invertir el orden de las filas (BMP las almacena de abajo hacia arriba)
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let bgr = [(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8]; // Extraer los componentes RGB del píxel

            file.write_all(&bgr)?; // Escribir el píxel en el archivo
        }

        file.write_all(&padding[..padding_size])?; // Añadir el padding necesario
    }

    Ok(())
}

// Definir un trait para renderizar el buffer como un archivo BMP
pub trait WriteBmp {
    fn render_buffer(&self, file_path: &str) -> std::io::Result<()>;
}

// Implementar el trait para el Framebuffer
impl WriteBmp for Framebuffer {
    fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}