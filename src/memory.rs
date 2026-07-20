pub struct Memory {
    pub ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        // initialize ram to all zeros, load font set at 0x50
        Memory { ram: [0 ; 4096] }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
        // this is where std::fs::read(path)? and copy_from_slice go

        println!("{}", path);
        Ok(())
    }
}