use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::player::Player;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 20.0;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    let mut new_x = player.position.x;
    let mut new_y = player.position.y;

    if window.is_key_down(Key::Up) {
        new_x += player.a.cos() * MOVE_SPEED;
        new_y += player.a.sin() * MOVE_SPEED;
    }
    if window.is_key_down(Key::Down) {
        new_x -= player.a.cos() * MOVE_SPEED;
        new_y -= player.a.sin() * MOVE_SPEED;
    }

    // Verificar si la nueva posición no está dentro de una pared
    let i = (new_x as usize) / block_size;
    let j = (new_y as usize) / block_size;

    if maze[j][i] == ' ' {
        player.position.x = new_x;
        player.position.y = new_y;
    }
}
