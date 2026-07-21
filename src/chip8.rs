use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Chip8 {
    pub memory: Memory,
    pub cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load_rom(path)
    }

    pub fn fetch(&mut self) -> u16 {
        self.cpu.fetch(&self.memory)
    }

    pub fn decode_and_execute(&mut self, opcode: u16) {
        self.cpu.decode_and_execute(opcode, &mut self.memory);
    }
}
