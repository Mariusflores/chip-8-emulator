mod chip8;
mod cpu;
mod display;
mod memory;
use chip8::Chip8;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(
        "Chip-8",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )?;

    let mut chip8 = Chip8::new();
    chip8.load_rom("./roms/ibm-logo.ch8")?;

    while window.is_open() {
        let opcode = chip8.fetch();
        chip8.decode_and_execute(opcode);
        
        let buffer = chip8.to_buffer(SCALE, WIDTH, HEIGHT); 
        window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)?;

    }

    #[allow(unreachable_code)]
    Ok(())
}
