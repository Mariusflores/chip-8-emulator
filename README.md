# Chip-8 Emulator


### Structure

src/
├── main.rs       // window/event loop, glue code
├── chip8.rs      // top-level struct owning cpu + memory + display + keypad
├── cpu.rs        // registers, PC, SP, stack, fetch/decode/execute
├── memory.rs     // [u8; 4096], ROM loading, font set
├── display.rs    // 64x32 framebuffer, draw logic
└── keypad.rs     // 16-key input state