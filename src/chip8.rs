pub mod Chip8 {
    const START_ADDR: u16 = 0x200;

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
    }

    impl Chip8 {
        pub fn new() -> Self {
            Chip8 {
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
            }
        }

        fn load_rom(&mut self, file: &str) {
            let bin_dat = std::fs::read(file).unwrap();
            for (i,&byte) in bin_dat.iter().enumerate() {
                self.memory[i] = byte;
            }
        }
    }
}

