use std::usize;

use crate::HEIGHT;

pub struct Display {
    pub pixels: [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [false; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [false; 64 * 32];
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;
        for (index, byte) in sprite.iter().enumerate() {
            for bit_position in (0..8).rev() {
                let screen_x = (x as usize + (7 - bit_position)) % 64;
                let screen_y = (y as usize + index) % 32;

                let pixel_index = screen_y * 64 + screen_x;

                if (byte >> bit_position) & 1 == 1 {
                    if self.pixels[pixel_index] {
                        collision = true;
                    }
                    self.pixels[pixel_index] ^= true;
                }
            }
        }

        return collision;
    }

    pub fn to_buffer(&self, scale: usize, width: usize, height: usize) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![0; width * scale * height * scale];

        for row in 0..height {
            for col in 0..width {
                let pixel = self.pixels[row * width + col];
                let color = if pixel { 0xFFFFFF } else { 0x000000 };

                for dy in 0..scale {
                    for dx in 0..scale {
                        let buffer_x = col * scale + dx;
                        let buffer_y = row * scale + dy;
                        let index = buffer_y * (width * scale) + buffer_x;
                        buffer[index] = color;
                    }
                }
            }
        }

        return buffer;
    }
}
