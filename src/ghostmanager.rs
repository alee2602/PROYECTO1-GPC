use crate::enemy::Enemy;
use nalgebra_glm::Vec2;
use std::time::{Instant, Duration};
use rand::Rng;

pub struct GhostManager {
    respawn_timer: Instant,
    respawn_duration: Duration,
}

impl GhostManager {
    pub fn new() -> Self {
        Self {
            respawn_timer: Instant::now(),
            respawn_duration: Duration::from_secs(7), 
        }
    }

    pub fn update_ghosts(&mut self, player_position: Vec2, maze: &Vec<Vec<char>>, enemies: &mut Vec<Enemy>, block_size: usize) {
        // Solo respawnear los fantasmas cuando el temporizador expira
        if self.respawn_timer.elapsed() >= self.respawn_duration {
            self.respawn_timer = Instant::now(); // Reiniciar el temporizador

            let mut rng = rand::thread_rng();

            for enemy in enemies.iter_mut() {
                // Elegir una nueva posición cercana al jugador
                loop {
                    let min_distance = block_size as f32 * 3.0; // Distancia mínima al jugador
                    let max_distance = block_size as f32 * 9.0; // Distancia máxima al jugador

                    let distance = rng.gen_range(min_distance..max_distance);
                    let angle = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));

                    // Calcular nuevas posiciones basadas en la distancia y el ángulo
                    let x = player_position.x + distance * angle.cos();
                    let y = player_position.y + distance * angle.sin();

                    let i = (x as usize) / block_size;
                    let j = (y as usize) / block_size;

                    // Verificar que la nueva posición es válida
                    if i < maze[0].len() && j < maze.len() && maze[j][i] == ' ' {
                        enemy.position.x = x;
                        enemy.position.y = y;
                        break;
                    }
                }
            }
        }
    }
}