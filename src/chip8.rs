extern crate rand;

use rand::Rng;

pub mod Chip8 {
    use rand::Rng;

    const START_ADDR: u16 = 0x200;
    const FONTSET_SIZE: usize = 80;
    const FONTSET_START_ADDR: usize = 0x50;
    const FONTSET: [u8; FONTSET_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    struct Chip8 {
        registers: [u8; 16],
        memory: [u8; 4096],
        index: u16,
        pc: u16,
        stack: [u16; 16],
        sp: u8,
        delay_timer: u8,
        sound_timer: u8,
        video: [u32; 2048],
        opcode: u16,
        rand_num: u8,
    }

    impl Chip8 {
        pub fn new() -> Self {
            let mut chip8 = Chip8 {
                registers: [0; 16],
                memory: [0; 4096],
                index: 0,
                pc: START_ADDR,
                stack: [0; 16],
                sp: 0,
                delay_timer: 0,
                sound_timer: 0,
                video: [0; 2048],
                opcode: 0,
                rand_num: rand::rng().random(),
            };

            for i in 0..FONTSET_SIZE {
                chip8.memory[i + FONTSET_START_ADDR] = FONTSET[i];
            }

            chip8
        }

        fn load_rom(&mut self, file: &str) {
            let bin_dat = std::fs::read(file).unwrap();
            for (i,&byte) in bin_dat.iter().enumerate() {
                self.memory[i] = byte;
            }
        }
    }
}

