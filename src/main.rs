use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

mod color;
mod controller;
mod framebuffer;
mod maze;
mod minimap;
mod player;
mod raycaster;
mod texture;

use crate::color::Color;
use crate::controller::process_events;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::minimap::render_minimap;
use crate::player::Player;
use crate::raycaster::cast_ray;
use crate::texture::Texture;

fn render2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    textures: [&Texture; 3],
    block_size: usize,
) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let xo = col * block_size;
            let yo = row * block_size;

            let texture = match maze[row][col] {
                '+' => Some(&textures[0]),
                '-' => Some(&textures[1]),
                '|' => Some(&textures[2]),
                _ => None,
            };

            if let Some(texture) = texture {
                for x in 0..block_size {
                    for y in 0..block_size {
                        let texture_x = (x * texture.width) / block_size;
                        let texture_y = (y * texture.height) / block_size;
                        let color = texture.get_pixel(texture_x, texture_y);
                        framebuffer.set_current_color(color);
                        framebuffer.point(xo + x, yo + y);
                    }
                }
            } else {
                framebuffer.set_current_color(Color::ground().to_hex());
                for x in 0..block_size {
                    for y in 0..block_size {
                        framebuffer.point(xo + x, yo + y);
                    }
                }
            }
        }
    }
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, angle, block_size, true);
    }

    // Dibujar al jugador como un punto en 2D
    framebuffer.set_current_color(Color::red().to_hex());
    framebuffer.point(player.position.x as usize, player.position.y as usize);
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, textures: [&Texture; 3]) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let num_rays = framebuffer.width; // Número de rayos basados en la anchura del framebuffer

    //let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    for y in 0..hh as usize {
        let ratio = y as f32 / hh; // Ratio de la posición actual en la altura del cielo
        let sky_color = Color::gradient_sky(ratio).to_hex();
        framebuffer.set_current_color(sky_color);
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Establecer el color del suelo
    framebuffer.set_current_color(Color::ground().to_hex());
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Cálculo de la altura de la pared
        let distance_to_wall = intersect.distance * (a - player.a).cos();
        let distance_to_projection_plane = 40.0;

        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        let texture = match intersect.impact {
            '+' => &textures[0],
            '-' => &textures[1],
            '|' => &textures[2],
            _ => continue,
        };

        let texture_x = (intersect.distance % texture.width as f32) as usize;

        for y in stake_top..stake_bottom {
            let texture_y = ((y - stake_top) * texture.height) / (stake_bottom - stake_top);
            let color = texture.get_pixel(texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}

fn main() {

    let (_stream, stream_handle) =
        OutputStream::try_default().expect("No se pudo inicializar el stream de audio.");
    let sink = Sink::try_new(&stream_handle).expect("No se pudo crear el sink de audio.");

    let file = File::open("assets/epiphanyts.wav").expect("No se pudo abrir el archivo de música.");
    let source =
        Decoder::new(BufReader::new(file)).expect("No se pudo decodificar el archivo de música.");

    sink.append(source.repeat_infinite());
    sink.play();

    let window_width = 50 * 25;
    let window_height = 50 * 17;
    let framebuffer_width = window_width;
    let framebuffer_height = window_height;
    let close_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Renderized Maze ",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(Color::white().to_hex());

    let mut player = Player {
        position: Vec2::new(100.0, 200.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let minimap_size = 200;
    let wall_texture1 = Texture::from_file("assets/texture1.jpg");
    let wall_texture2 = Texture::from_file("assets/texture2.jpg");
    let wall_texture3 = Texture::from_file("assets/texture3.jpg");

    let textures = [&wall_texture1, &wall_texture2, &wall_texture3];

    let mut mode = "2D"; // Modo inicial

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&mut window, &mut player, &maze, block_size);
        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, textures, block_size);
        } else {
            render3d(&mut framebuffer, &player, textures);
            render_minimap(
                &mut framebuffer,
                &player,
                &maze,
                minimap_size,
                block_size,
                textures,
            );
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(close_delay);
    }
}
