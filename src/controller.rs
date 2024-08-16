use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::player::Player;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 7.0;
    const ROTATION_SPEED: f32 = PI / 15.0;

    // Rotación con las teclas A y D
    if window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
    }

    let mut new_x = player.position.x;
    let mut new_y = player.position.y;

    // Movimiento hacia adelante con W
    if window.is_key_down(Key::W) {
        new_x += player.a.cos() * MOVE_SPEED;
        new_y += player.a.sin() * MOVE_SPEED;
    }
    // Movimiento hacia atrás con S
    if window.is_key_down(Key::S) {
        new_x -= player.a.cos() * MOVE_SPEED;
        new_y -= player.a.sin() * MOVE_SPEED;
    }

    // Movimiento lateral hacia la izquierda con Q
    if window.is_key_down(Key::Q) {
        new_x -= player.a.sin() * MOVE_SPEED;
        new_y += player.a.cos() * MOVE_SPEED;
    }
    // Movimiento lateral hacia la derecha con E
    if window.is_key_down(Key::E) {
        new_x += player.a.sin() * MOVE_SPEED;
        new_y -= player.a.cos() * MOVE_SPEED;
    }

    // Verificar si la nueva posición no está dentro de una pared
    let i = (new_x as usize) / block_size;
    let j = (new_y as usize) / block_size;

    if maze[j][i] == ' ' {
        player.position.x = new_x;
        player.position.y = new_y;
    }
}
