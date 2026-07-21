use crate::display::Display;
use crate::memory::Memory;
pub struct Cpu {
    pub registers: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u16,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            i: 0,
            pc: 0x200, // Programs start here
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u16 {
        let high = memory.ram[self.pc as usize] as u16;
        let low = memory.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        (high << 8) | low
    }

    pub fn decode_and_execute(&mut self, opcode: u16, memory: &mut Memory, display: &mut Display) {
        let nibble1 = (opcode & 0xF000) >> 12;

        match nibble1 {
            0x0 => {
                // 00E0 = clear screen, 00EE = return — check the last byte to tell them apart
                match opcode & 0x00FF {
                    0xE0 => display.clear(),
                    0xEE => println!("return"),
                    _ => println!("unhandled 0x0 opcode: {:#06x}", opcode),
                }
            }
            0x1 => {
                // 1NNN = jump
                let nnn = opcode & 0x0FFF;
                self.pc = nnn;
            }
            0x6 => {
                // 6XNN = set Vx
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x] = nn
            }
            0x7 => {
                // 7XNN = add Vx
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }
            0xA => {
                // ANNN = set I
                let nnn = opcode & 0x0FFF;
                self.i = nnn;
            }
            0xD => {
                // DXYN = draw — stub as a print for now, no Display yet
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let n = (opcode & 0x000F) as u8;
                let sprite = &memory.ram[self.i as usize..(self.i as usize + n as usize)];
                let collision = display.draw(x, y, sprite);
                self.registers[0xF] = if collision { 1 } else { 0 };
            }
            _ => println!("unhandled opcode: {:#06x}", opcode),
        }
    }
}
