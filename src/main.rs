mod chip8;
mod cpu;
mod memory;
use chip8::Chip8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chip8 = Chip8::new();
    chip8.load_rom("./roms/ibm-logo.ch8")?;
    // print/inspect memory here to verify it loaded
    Ok(())
}
