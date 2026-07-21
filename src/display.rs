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
        for (index, byte) in sprite.iter().enumerate() {
            for bit_position in (0..8).rev() {
                println!("{}", bit_position);
                (byte >> bit_position) & 1;
                // TODO implement actual draw functionality
            }
        }

        return true;
    }
}
