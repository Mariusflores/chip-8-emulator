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
}
