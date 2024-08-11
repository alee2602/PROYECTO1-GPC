use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::color::Color;

pub struct Intersect{
    pub distance: f32,
    pub impact: char
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>, 
    player: &Player, 
    angle: f32,
    block_size: usize, 
    draw_line: bool,
) -> Intersect{
    let mut d = 0.0;

    if draw_line {
        framebuffer.set_current_color(Color::red().to_hex());
    }

    loop {
        let cos_angle = d * angle.cos();
        let sin_angle = d * angle.sin();
        let x = (player.position.x + cos_angle) as usize;
        let y = (player.position.y + sin_angle) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            return Intersect{
                distance:d,
                impact: maze[j][i]
            }
        }
        if draw_line{
            framebuffer.point(x, y);
        }

        d += 1.0;
    }
}
