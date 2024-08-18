use nalgebra_glm::Vec2;

pub struct Enemy {
    pub position: Vec2,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }

    pub fn check_collision_with_player(&self, player_pos: &Vec2, distance_threshold: f32) -> bool {
        // Comprobar si el enemigo está lo suficientemente cerca del jugador como para colisionar
        (self.position - player_pos).norm() < distance_threshold
    }
}
