use std::time::{Instant, Duration};
use crate::framebuffer::Framebuffer;
use crate::color::Color;

pub struct FPSCounter {
    last_instant: Instant,
    frame_count: usize,
    fps: usize,
}

impl FPSCounter {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, x: usize, y: usize, scale: usize) {
        let fps_string = format!("FPS: {}", self.fps);
        render_text(framebuffer, &fps_string, x, y, scale);
    }
}

fn render_text(framebuffer: &mut Framebuffer, text: &str, x: usize, y: usize, scale: usize) {
    
    let mut offset_x = x;
    
    for ch in text.chars() {
        if let Some(digit) = ch.to_digit(10) {
            render_digit(framebuffer, digit as usize, offset_x, y, scale);
        } else if ch == ':' || ch == ' ' || ch == 'F' || ch == 'P' || ch == 'S' {
            // Render placeholders for FPS letters and colon
            render_symbol(framebuffer, ch, offset_x, y, scale);
        }
        offset_x += (3 * scale) + scale;
    }
}

fn render_digit(framebuffer: &mut Framebuffer, digit: usize, x: usize, y: usize, scale: usize) {
    const DIGIT_MAP: [[u8; 15]; 10] = [
        // Dígito 0
        [
            1, 1, 1, // ###
            1, 0, 1, // # #
            1, 0, 1, // # #
            1, 0, 1, // # #
            1, 1, 1, // ###
        ],
        // Dígito 1
        [
            0, 1, 0, //  #
            1, 1, 0, // ##
            0, 1, 0, //  #
            0, 1, 0, //  #
            1, 1, 1, // ###
        ],
        // Dígito 2
        [
            1, 1, 1, // ###
            0, 0, 1, //   #
            1, 1, 1, // ###
            1, 0, 0, // #  
            1, 1, 1, // ###
        ],
        // Dígito 3
        [
            1, 1, 1, // ###
            0, 0, 1, //   #
            1, 1, 1, // ###
            0, 0, 1, //   #
            1, 1, 1, // ###
        ],
        // Dígito 4
        [
            1, 0, 1, // # #
            1, 0, 1, // # #
            1, 1, 1, // ###
            0, 0, 1, //   #
            0, 0, 1, //   #
        ],
        // Dígito 5
        [
            1, 1, 1, // ###
            1, 0, 0, // #  
            1, 1, 1, // ###
            0, 0, 1, //   #
            1, 1, 1, // ###
        ],
        // Dígito 6
        [
            1, 1, 1, // ###
            1, 0, 0, // #  
            1, 1, 1, // ###
            1, 0, 1, // # #
            1, 1, 1, // ###
        ],
        // Dígito 7
        [
            1, 1, 1, // ###
            0, 0, 1, //   #
            0, 0, 1, //   #
            0, 0, 1, //   #
            0, 0, 1, //   #
        ],
        // Dígito 8
        [
            1, 1, 1, // ###
            1, 0, 1, // # #
            1, 1, 1, // ###
            1, 0, 1, // # #
            1, 1, 1, // ###
        ],
        // Dígito 9
        [
            1, 1, 1, // ###
            1, 0, 1, // # #
            1, 1, 1, // ###
            0, 0, 1, //   #
            1, 1, 1, // ###
        ]
    ];

    let color = Color::white().to_hex();

    for row in 0..5 {
        for col in 0..3 {
            if DIGIT_MAP[digit][row * 3 + col] == 1 {
                for i in 0..scale {
                    for j in 0..scale {
                        framebuffer.set_current_color(color);
                        framebuffer.point(x + col * scale + i, y + row * scale + j);
                    }
                }
            }
        }
    }
}

fn render_symbol(framebuffer: &mut Framebuffer, symbol: char, x: usize, y: usize, scale: usize) {
    const SYMBOL_MAP: [([u8; 15], char); 4] = [
        ([1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0], 'F'), // F
        ([1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0], 'P'), // P
        ([1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1], 'S'), // S
        ([0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], ':'), // ':'
    ];

    let color = Color::white().to_hex();

    if let Some((symbol_data, _)) = SYMBOL_MAP.iter().find(|&&(_, s)| s == symbol) {
        for row in 0..5 {
            for col in 0..3 {
                if symbol_data[row * 3 + col] == 1 {
                    for i in 0..scale {
                        for j in 0..scale {
                            framebuffer.set_current_color(color);
                            framebuffer.point(x + col * scale + i, y + row * scale + j);
                        }
                    }
                }
            }
        }
    }
}
