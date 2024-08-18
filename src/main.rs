use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

mod color;
mod controller;
mod enemy;
mod fps;
mod framebuffer;
mod ghostmanager;
mod maze;
mod minimap;
mod player;
mod raycaster;
mod texture;

use crate::color::Color;
use crate::controller::process_events;
use crate::enemy::Enemy;
use crate::framebuffer::Framebuffer;
use crate::ghostmanager::GhostManager;
use crate::maze::load_maze;
use crate::minimap::render_minimap;
use crate::player::Player;
use crate::raycaster::cast_ray;
use crate::texture::Texture;
use fps::FPSCounter;

enum GameState {
    StartScreen,
    Playing,
    Victory,
    Defeat,
}

fn render2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    textures: [&Texture; 3],
    block_size: usize,
) {
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

    framebuffer.set_current_color(Color::red().to_hex());
    framebuffer.point(player.position.x as usize, player.position.y as usize);
}

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    textures: [&Texture; 3],
    ghost_texture: &Texture,
    enemies: &Vec<Enemy>,
    scale_factor: usize,
) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    for y in 0..hh as usize {
        let ratio = y as f32 / hh;
        let sky_color = Color::gradient_sky(ratio).to_hex();
        framebuffer.set_current_color(sky_color);
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    framebuffer.set_current_color(Color::ground().to_hex());
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Renderizado de las paredes con texturas escaladas
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

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

        let texture_x = (intersect.distance % (texture.width / scale_factor) as f32) as usize;

        for y in stake_top..stake_bottom {
            let texture_y = ((y - stake_top) * (texture.height / scale_factor)) / (stake_bottom - stake_top);
            let color = texture.get_pixel(texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }

    // Límite máximo de distancia para renderizar fantasmas
    let max_render_distance = 250.0;
    for enemy in enemies.iter() {
        let dx = enemy.position.x - player.position.x;
        let dy = enemy.position.y - player.position.y;
        let distance_to_enemy = (dx * dx + dy * dy).sqrt();

        // No renderizar fantasmas que estén más lejos que el límite
        if distance_to_enemy > max_render_distance {
            continue;
        }

        if distance_to_enemy > 0.5 {
            let enemy_angle = (dy).atan2(dx);
            let relative_angle = enemy_angle - player.a;

            // Verificar si el fantasma está en el campo de visión
            if relative_angle.abs() < player.fov / 2.0 {
                let intersect =
                    cast_ray(framebuffer, &maze, &player, enemy_angle, block_size, false);

                if intersect.distance < distance_to_enemy {
                    continue;
                }
                let distance_to_projection_plane = 80.0;
                let enemy_height = (hh / distance_to_enemy) * distance_to_projection_plane;
                let enemy_top = (hh - (enemy_height / 2.0)) as usize;
                let enemy_bottom = (hh + (enemy_height / 2.0)) as usize;

                let enemy_screen_position = (framebuffer.width as f32 / 2.0)
                    + (relative_angle / player.fov) * framebuffer.width as f32;
                let enemy_screen_position = enemy_screen_position as usize;

                let ghost_width = ghost_texture.width;
                let ghost_height = ghost_texture.height;

                for y in enemy_top..enemy_bottom {
                    let texture_y = ((y - enemy_top) * ghost_height) / (enemy_bottom - enemy_top);
                    let mut x_offset = 0;

                    for x in enemy_screen_position..(enemy_screen_position + ghost_width) {
                        if x >= framebuffer.width {
                            continue;
                        }

                        let texture_x = (x_offset * ghost_texture.width) / ghost_width;
                        let color = ghost_texture.get_pixel(texture_x, texture_y);

                        //Renderizar el píxel solo si no es transparente
                        let r = ((color >> 24) & 0xFF) as u8;
                        let g = ((color >> 16) & 0xFF) as u8;
                        let b = ((color >> 8) & 0xFF) as u8;
                        let a = (color & 0xFF) as u8;

                        // Solo renderizar el píxel si no es completamente transparente
                        if a > 0 {
                            framebuffer.set_current_color(Color::new(r, g, b).to_hex());
                            framebuffer.point(x, y);
                        }
                        x_offset += 1;
                    }
                }
            }
        }
    }
}

//Renderizar pantalla de inicio
fn render_start_screen(framebuffer: &mut Framebuffer, start_texture: &Texture) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let texture_x = (x * start_texture.width) / framebuffer.width;
            let texture_y = (y * start_texture.height) / framebuffer.height;
            let color = start_texture.get_pixel(texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

//Renderizar pantalla de éxito
fn render_victory_screen(framebuffer: &mut Framebuffer, victory_texture: &Texture) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let texture_x = (x * victory_texture.width) / framebuffer.width;
            let texture_y = (y * victory_texture.height) / framebuffer.height;
            let color = victory_texture.get_pixel(texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

//Renderizar pantalla de derrota
fn render_defeat_screen(framebuffer: &mut Framebuffer, defeat_texture: &Texture) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let texture_x = (x * defeat_texture.width) / framebuffer.width;
            let texture_y = (y * defeat_texture.height) / framebuffer.height;
            let color = defeat_texture.get_pixel(texture_x, texture_y);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn player_reached_end(player_position: &Vec2) -> bool {
    let end_position = Vec2::new(524.0, 573.0); // Coordenadas del final
    (player_position - end_position).norm() < 10.0 // Si está cerca del final
}

fn ghost_touched_player(enemies: &Vec<Enemy>, player_position: &Vec2) -> bool {
    for enemy in enemies {
        if enemy.check_collision_with_player(player_position, 10.0) {
            return true;
        }
    }
    false
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

    let window_width = 50 * 19;
    let window_height = 50 * 13;
    let framebuffer_width = window_width;
    let framebuffer_height = window_height;
    let mut close_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Whispers of Epiphany",
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
    let wall_texture2 = Texture::from_file("assets/texture3.jpg");
    let wall_texture3 = Texture::from_file("assets/texture2.jpg");
    let ghost_texture = Texture::from_file("assets/ghost.png");

    let start_texture = Texture::from_file("assets/woe.jpg");
    let victory_texture = Texture::from_file("assets/won.jpg");
    let defeat_texture = Texture::from_file("assets/failed.jpg");

    let textures = [&wall_texture1, &wall_texture2, &wall_texture3];

    // Crear enemigos en posiciones válidas cerca del jugador
    let mut rng = rand::thread_rng();
    let mut enemies = vec![];
    let player_start_position = Vec2::new(100.0, 200.0);

    for _ in 0..5 {
        loop {
            let x = rng.gen_range(1..maze[0].len()) as f32 * block_size as f32;
            let y = rng.gen_range(1..maze.len()) as f32 * block_size as f32;

            let i = (x as usize) / block_size;
            let j = (y as usize) / block_size;

            if maze[j][i] == ' '
                && (Vec2::new(x, y) - player_start_position).norm() > block_size as f32
            {
                enemies.push(Enemy::new(x, y));
                break;
            }
        }
    }

    let mut ghost_manager = GhostManager::new();
    let mut fps_counter = FPSCounter::new();

    let mut game_state = GameState::StartScreen;
    let mut mode = "2D"; // Modo inicial

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();

        fps_counter.update();
        match game_state {
            GameState::StartScreen => {
                render_start_screen(&mut framebuffer, &start_texture);
                if window.is_key_down(Key::Enter) {
                    game_state = GameState::Playing;
                    player.position = player_start_position;
                }
            }
            GameState::Playing => {
                // Cambiar entre los modos 2D y 3D
                if window.is_key_down(Key::M) {
                    mode = if mode == "2D" { "3D" } else { "2D" };
                }

                process_events(&mut window, &mut player, &maze, block_size);
                framebuffer.clear();

                // Lógica de renderizado para 2D o 3D
                if mode == "2D" {
                    render2d(&mut framebuffer, &player, &maze, textures, block_size);
                } else {
                    render3d(
                        &mut framebuffer,
                        &player,
                        textures,
                        &ghost_texture,
                        &enemies,
                        1
                    );
                    render_minimap(
                        &mut framebuffer,
                        &player,
                        &maze,
                        minimap_size,
                        block_size,
                        textures
                    );
                    ghost_manager.update_ghosts(player.position, &maze, &mut enemies, block_size);
                    
                }
                fps_counter.render(&mut framebuffer, 10, 10, 2);

                // Verificar si el jugador ha ganado o perdido
                if player_reached_end(&player.position) {
                    game_state = GameState::Victory;
                } else if ghost_touched_player(&enemies, &player.position) {
                    game_state = GameState::Defeat;
                }

                // Renderizado y lógica de los fantasmas
                for enemy in &mut enemies {
                    if enemy.check_collision_with_player(&player.position, block_size as f32 / 2.0)
                    {
                        game_state = GameState::Defeat;
                    }
                }
            }
            GameState::Victory => {
                render_victory_screen(&mut framebuffer, &victory_texture);
                if window.is_key_down(Key::Enter) {
                    std::process::exit(0);
                }
            }
            GameState::Defeat => {
                render_defeat_screen(&mut framebuffer, &defeat_texture);
                if window.is_key_down(Key::Enter) {
                    game_state = GameState::StartScreen;
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // Ajustar el tiempo de espera dinámicamente basado en el rendimiento
        let frame_time = start_time.elapsed();
        if frame_time < close_delay {
            std::thread::sleep(close_delay - frame_time);
        } else {
            close_delay = Duration::from_millis(16); 
        }
    }
}
