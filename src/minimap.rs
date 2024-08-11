use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::color::Color;
use crate::texture::Texture;

pub fn render_minimap(
    framebuffer: &mut Framebuffer, 
    player: &Player, 
    maze: &Vec<Vec<char>>, 
    minimap_size: usize, 
    block_size: usize,
    textures: [&Texture; 3],
) {
    let minimap_x_offset = framebuffer.width - minimap_size - 10; 
    let minimap_y_offset = 10; 
    // Dibujar el fondo del minimapa
    framebuffer.set_current_color(Color::folklore_ground().to_hex());
    for x in minimap_x_offset..minimap_x_offset + minimap_size {
        for y in minimap_y_offset..minimap_y_offset + minimap_size {
            framebuffer.point(x, y);
        }
    }

    let scale = minimap_size as f32 / (maze.len() as f32 * block_size as f32);

    // Dibujar las paredes del laberinto en el minimapa
    for (row, maze_row) in maze.iter().enumerate() {
        for (col, cell) in maze_row.iter().enumerate() {
            if *cell != ' ' {
                let texture = match cell {
                    '+' => Some(&textures[0]), // Texture for '+'
                    '-' => Some(&textures[1]), // Texture for '-'
                    '|' => Some(&textures[2]), // Texture for '|'
                    _ => None,
                };

                if let Some(texture) = texture {
                    let x = minimap_x_offset + ((col * block_size) as f32 * scale) as usize;
                    let y = minimap_y_offset + ((row * block_size) as f32 * scale) as usize;
                    
                    for dx in 0..(block_size as f32 * scale) as usize {
                        for dy in 0..(block_size as f32 * scale) as usize {
                            let texture_x = ((dx as f32 / scale) * texture.width as f32 / block_size as f32) as usize;
                            let texture_y = ((dy as f32 / scale) * texture.height as f32 / block_size as f32) as usize;
                            let color = texture.get_pixel(texture_x, texture_y);
                            framebuffer.set_current_color(color);
                            framebuffer.point(x + dx, y + dy);
                        }
                    }
                } else {
                    // Si no hay textura definida, colorea la celda con un color sólido
                    framebuffer.set_current_color(Color::folklore_ground().to_hex());
                    let x = minimap_x_offset + ((col * block_size) as f32 * scale) as usize;
                    let y = minimap_y_offset + ((row * block_size) as f32 * scale) as usize;
                    for dx in 0..(block_size as f32 * scale) as usize {
                        for dy in 0..(block_size as f32 * scale) as usize {
                            framebuffer.point(x + dx, y + dy);
                        }
                    }
                }
            }
        }
    }

    // Dibujar la posición del jugador en el minimapa
    framebuffer.set_current_color(Color::red().to_hex());
    let player_minimap_x = minimap_x_offset + (player.position.x * scale) as usize;
    let player_minimap_y = minimap_y_offset + (player.position.y * scale) as usize;

    framebuffer.point(player_minimap_x, player_minimap_y);
    framebuffer.point(player_minimap_x + 1, player_minimap_y);
    framebuffer.point(player_minimap_x, player_minimap_y + 1);
    framebuffer.point(player_minimap_x + 1, player_minimap_y + 1);
}