mod chip8;
mod cpu;
mod display;
mod memory;
use chip8::Chip8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chip8 = Chip8::new();
    chip8.load_rom("./roms/ibm-logo.ch8")?;

    loop {
        let opcode = chip8.fetch();
        chip8.decode_and_execute(opcode);
    }

    #[allow(unreachable_code)]
    Ok(())
}
