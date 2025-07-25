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
    const VIDEO_WIDTH: u8 = 64;
    const VIDEO_HEIGHT: u8 = 32;

    struct Chip8 {
        registers: [u8; 16],
        memory: [u8; 4096],
        index: u16,
        pc: u16,
        stack: [u16; 16],
        sp: u8,
        delay_timer: u8,
        sound_timer: u8,
        keypad: [u8; 16],
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
                keypad: [0; 16],
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

        // CLS: Clear screen
        fn OP_00E0(&mut self) {
            self.video
                .iter_mut()
                .for_each(|m| *m = 0);
        }

        // RET: Return value
        fn OP_00EE(&mut self) {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }

        // JMP: Jump to addr.
        fn OP_1NNN(&mut self) {
            self.pc = self.opcode & 0x0FFF;
        }

        // CALL: Call routine at addr.
        fn OP_2NNN(&mut self) {
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = self.opcode & 0x0FFF;
        }

        // Skip if Vx == kk
        fn OP_3XKK(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            if self.registers[vx] == kk {
                self.pc += 2;
            }
        }

        // Skip if Vx != kk
        fn OP_4XKK(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            if self.registers[vx] != kk {
                self.pc += 2;
            }
        }

        // Skip if Vx == Vy
        fn OP_5XY0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.registers[vx] == self.registers[vy] {
                self.pc += 2;
            }
        }

        // Set Vx = kk
        fn OP_6XKK(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] = kk;
        }

        // Set Vx += kk
        fn OP_7XKK(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] += kk;
        }

        // Set Vx = Vy
        fn OP_8XY0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] += self.registers[vy];
        }

        // Set Vx |= Vy
        fn OP_8XY1(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] |= self.registers[vy];
        }

        // Set Vx &= Vy
        fn OP_8XY2(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] &= self.registers[vy];
        }

        // Set Vx ^= Vy
        fn OP_8XY3(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] ^= self.registers[vy];
        }

        // Set Vx += Vy, VF to borrow
        fn OP_8XY4(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            let sum = vx + vy;
            self.registers[0xF] = if sum > 255 {1} else {0};
            self.registers[vx] = sum as u8 & 0xFF;

        }

        // Set Vx -= Vy, VF to NOT borrow
        fn OP_8XY5(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[0xF] = (self.registers[vx] > self.registers[vy]) as u8;
            self.registers[vx] -= self.registers[vy];
        }

        // Set Vx = Vx >> 1, VF to lost bit
        fn OP_8XY6(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[0xF] = self.registers[vx] & 0x1;
            self.registers[vx] >>= 1;
        }

        // Set Vx = Vy - Vx to NOT borrow
        fn OP_8XY7(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[0xF] = (self.registers[vx] < self.registers[vy]) as u8;
            self.registers[vx] = self.registers[vy] - self.registers[vx];
        }

        // Set Vx = Vx << 1, VF to lost bit
        fn OP_8XYE(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;
            self.registers[vx] <<= 1;
        }

        // Skip next inst if vx != vy
        fn OP_9XY0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.registers[vx] != self.registers[vy] {
                self.pc += 2;
            }
        }

        // Set I == nnn
        fn OP_ANNN(&mut self) {
            self.index = self.opcode & 0x0FFF;
        }

        // Jump to nnn + V0
        fn OP_BNNN(&mut self) {
            self.pc += self.opcode & 0x0FFF;
        }

        // Vx = RND & kk
        fn OP_CXKK(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] += kk;
        }

        // Display sprite at Vx, Vy, set VF = collision
        fn OP_DXYN(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            let height = self.opcode & 0x000F;

            let xpos = self.registers[vx] % VIDEO_WIDTH;
            let ypos = self.registers[vy] % VIDEO_HEIGHT;
            self.registers[0xF] = 0;

            for row in 0..height {
                for col in 0..8 {
                    let sprite_pxl = self.memory[ (self.index + row) as usize] & (0x80 >> col);
                    let screen_y = (ypos as u16 + row) * VIDEO_WIDTH as u16;
                    let screen_x = (xpos + col) as u16;
                    let screen_pxl = &mut self.video[(screen_y + screen_x) as usize];
                    if sprite_pxl != 0 {
                        if (*screen_pxl == 0xFFFFFFFF) {
                            self.registers[0xF] = 1;
                        }
                        *screen_pxl ^= 0xFFFFFFFF;
                    }
                }
            }
        }

        // Skip if Vx == key
        fn OP_EX9E(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            if (self.keypad[self.registers[vx] as usize] != 0) {
                self.pc += 2;
            }
        }

        // Skip if Vx != key
        fn OP_EX91(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            if (self.keypad[self.registers[vx] as usize] == 0) {
                self.pc += 2;
            }
        }

        // Set vx to delay timer
        fn OP_FX07(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[vx] = self.delay_timer;
        }

        // Wait for a keypress and store the result in Vx
        fn OP_FX0A(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            for i in 0..16 {
                if (self.keypad[i] != 0) {
                    self.registers[vx] = i as u8;
                    return;
                }
            }
            self.pc -= 2;
        }
    }
}


