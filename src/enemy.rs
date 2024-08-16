use nalgebra_glm::Vec2;

pub struct Enemy {
    pub position: Vec2,
    pub speed: f32,
    pub direction: Vec2, 
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32, direction_x: f32, direction_y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            speed,
            direction: Vec2::new(direction_x, direction_y), 
        }
    }
    
    pub fn move_enemy(&mut self, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_position = self.position + self.direction * self.speed;

        // Verificar si el nuevo movimiento del enemigo colisionaría con una pared
        let i = (new_position.x as usize) / block_size;
        let j = (new_position.y as usize) / block_size;

        if maze[j][i] == ' ' {
            self.position = new_position;
        } else {
            self.direction = -self.direction;
        }
    }

    pub fn check_collision_with_player(&self, player_pos: &Vec2, distance_threshold: f32) -> bool {
        // Comprobar si el enemigo está lo suficientemente cerca del jugador como para colisionar
        (self.position - player_pos).norm() < distance_threshold
    }
}
