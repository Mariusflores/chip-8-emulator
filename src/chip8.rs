use crate::memory::Memory;

pub struct Chip8 {
    pub memory: Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load_rom(path)
    }
}
