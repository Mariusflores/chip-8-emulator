use crate::memory::Memory;
use crate::cpu::Cpu;

pub struct Chip8 {
    pub memory: Memory,
    pub cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load_rom(path)
    }
}
