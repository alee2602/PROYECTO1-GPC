use crate::enemy::Enemy;
use nalgebra_glm::Vec2;
use std::time::{Instant, Duration};
use rand::Rng;
use std::f32::consts::PI;

pub struct GhostManager {
    respawn_timer: Instant,
    respawn_duration: Duration,
}

impl GhostManager {
    pub fn new() -> Self {
        Self {
            respawn_timer: Instant::now(),
            respawn_duration: Duration::from_secs(5), 
        }
    }

    pub fn update_ghosts(
        &mut self,
        player_position: Vec2,
        maze: &Vec<Vec<char>>,
        enemies: &mut Vec<Enemy>,
        block_size: usize,
    ) {
        // Respawnear fantasmas cuando el temporizador expira
        if self.respawn_timer.elapsed() >= self.respawn_duration {
            self.respawn_timer = Instant::now(); // Reiniciar el temporizador

            let mut rng = rand::thread_rng();

            for enemy in enemies.iter_mut() {
                let min_distance = block_size as f32 * 5.0; 
                let max_distance = block_size as f32 * 10.0; 

                loop {
                    // Elegir una distancia aleatoria en el rango y un ángulo aleatorio
                    let distance = rng.gen_range(min_distance..max_distance);
                    let angle = rng.gen_range(0.0..(2.0 * PI));

                    // Calcular nuevas posiciones basadas en el ángulo y la distancia
                    let x = player_position.x + distance * angle.cos();
                    let y = player_position.y + distance * angle.sin();

                    let i = (x as usize) / block_size;
                    let j = (y as usize) / block_size;

                    // Si la nueva posición es válida y no es una pared, mover el fantasma ahí
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