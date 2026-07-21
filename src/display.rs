pub struct Display {
    pub pixels : [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [false; 64 * 32],
        }
    }

    pub fn clear(&mut self){
        self.pixels = [false; 64 * 32];
    }
    
}
