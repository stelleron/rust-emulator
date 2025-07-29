extern crate rand;
extern crate sdl2;

pub mod Chip8 {
    use sdl2::keyboard::Keycode;

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
    pub const VIDEO_WIDTH: u8 = 64;
    pub const VIDEO_HEIGHT: u8 = 32;

    pub struct Chip8 {
        registers: [u8; 16],
        memory: [u8; 4096],
        index: u16,
        pc: u16,
        stack: [u16; 16],
        sp: u8,
        delay_timer: u8,
        sound_timer: u8,
        keypad: [u8; 16],
        pub video: [u32; 2048],
        opcode: u16,
        rand_num: u8,
        table: [fn(&mut Chip8); 0xF + 1],
        table_0: [fn(&mut Chip8); 0xF + 1],
        table_8: [fn(&mut Chip8); 0xF + 1],
        table_e: [fn(&mut Chip8); 0xF + 1],
        table_f: [fn(&mut Chip8); 0xFF + 1],
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
                table: [Chip8::op_null; 0xF + 1],
                table_0: [Chip8::op_null; 0xF + 1],
                table_8: [Chip8::op_null; 0xF + 1],
                table_e: [Chip8::op_null; 0xF + 1],
                table_f: [Chip8::op_null; 0xFF + 1]
            };

            chip8.table[0x0] = Chip8::op_table_0;
            chip8.table[0x1] = Chip8::op_1nnn;
            chip8.table[0x2] = Chip8::op_2nnn;
            chip8.table[0x3] = Chip8::op_3xkk;
            chip8.table[0x4] = Chip8::op_4xkk;
            chip8.table[0x5] = Chip8::op_5xy0;
            chip8.table[0x6] = Chip8::op_6xkk;
            chip8.table[0x7] = Chip8::op_7xkk;
            chip8.table[0x8] = Chip8::op_table_8;
            chip8.table[0x9] = Chip8::op_9xy0;
            chip8.table[0xA] = Chip8::op_annn;
            chip8.table[0xB] = Chip8::op_bnnn;
            chip8.table[0xC] = Chip8::op_cxkk;
            chip8.table[0xD] = Chip8::op_dxyn;
            chip8.table[0xE] = Chip8::op_table_e;
            chip8.table[0xF] = Chip8::op_table_f;

            chip8.table_0[0x0] = Chip8::op_00e0;
            chip8.table_0[0xE] = Chip8::op_00ee;

            chip8.table_8[0x0] = Chip8::op_8xy0;
            chip8.table_8[0x1] = Chip8::op_8xy1;
            chip8.table_8[0x2] = Chip8::op_8xy2;
            chip8.table_8[0x3] = Chip8::op_8xy3;
            chip8.table_8[0x4] = Chip8::op_8xy4;
            chip8.table_8[0x5] = Chip8::op_8xy5;
            chip8.table_8[0x6] = Chip8::op_8xy6;
            chip8.table_8[0x7] = Chip8::op_8xy7;
            chip8.table_8[0xE] = Chip8::op_8xye;

            chip8.table_e[0x1] = Chip8::op_ex91;
            chip8.table_e[0xE] = Chip8::op_ex9e;

            chip8.table_f[0x07] = Chip8::op_fx07;
            chip8.table_f[0x0A] = Chip8::op_fx0a;
            chip8.table_f[0x15] = Chip8::op_fx15;
            chip8.table_f[0x18] = Chip8::op_fx18;
            chip8.table_f[0x1E] = Chip8::op_fx1e;
            chip8.table_f[0x29] = Chip8::op_fx29;
            chip8.table_f[0x33] = Chip8::op_fx33;
            chip8.table_f[0x55] = Chip8::op_fx55;
            chip8.table_f[0x65] = Chip8::op_fx65;

            for i in 0..FONTSET_SIZE {
                chip8.memory[i + FONTSET_START_ADDR] = FONTSET[i];
            }
            chip8
        }

        pub fn load_rom(&mut self, file: &str) {
            let bin_dat = std::fs::read(file).unwrap();
            for (i,&byte) in bin_dat.iter().enumerate() {
                self.memory[i + START_ADDR as usize] = byte;
            }
        }

        pub fn process_input(&mut self, code: Keycode, down: bool) {
            match code {
                Keycode::X => { self.keypad[0] = down as u8}
                Keycode::Num1 => { self.keypad[1] = down as u8}
                Keycode::Num2 => { self.keypad[2] = down as u8}
                Keycode::Num3 => { self.keypad[3] = down as u8}
                Keycode::Q => { self.keypad[4] = down as u8}
                Keycode::W => { self.keypad[5] = down as u8}
                Keycode::E => { self.keypad[6] = down as u8}
                Keycode::A => { self.keypad[7] = down as u8}
                Keycode::S => { self.keypad[8] = down as u8}
                Keycode::D => { self.keypad[9] = down as u8}
                Keycode::Z => { self.keypad[0xA] = down as u8}
                Keycode::C => { self.keypad[0xB] = down as u8}
                Keycode::Num4 => { self.keypad[0xC] = down as u8}
                Keycode::R => { self.keypad[0xD] = down as u8}
                Keycode::F => { self.keypad[0xE] = down as u8}
                Keycode::V => { self.keypad[0xF] = down as u8}
                _ => ()
            }
        }

        pub fn cycle(&mut self) {
            self.opcode = (self.memory[self.pc as usize] as u16) << 8;
            self.opcode |= self.memory[self.pc as usize + 1] as u16;

            self.pc += 2;
            self.table[(self.opcode as usize & 0xF000) >> 12](self);

            if self.delay_timer > 0 {self.delay_timer -= 1;}
            if self.sound_timer > 0 {self.sound_timer -= 1;}
        }

        // Table functions
        // ---------------
        fn op_null(&mut self) {}
        fn op_table_0(&mut self) {self.table_0[self.opcode as usize & 0x000F](self);}
        fn op_table_8(&mut self) {self.table_8[self.opcode as usize & 0x000F](self);}
        fn op_table_e(&mut self) {self.table_e[self.opcode as usize & 0x000F](self);}
        fn op_table_f(&mut self) {self.table_f[self.opcode as usize & 0x00FF](self);}

        // ---------------

        // CLS: Clear screen
        fn op_00e0(&mut self) {
            self.video
                .iter_mut()
                .for_each(|m| *m = 0);
        }

        // RET: Return value
        fn op_00ee(&mut self) {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }

        // JMP: Jump to addr.
        fn op_1nnn(&mut self) {
            self.pc = self.opcode & 0x0FFF;
        }

        // CALL: Call routine at addr.
        fn op_2nnn(&mut self) {
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = self.opcode & 0x0FFF;
        }

        // Skip if Vx == kk
        fn op_3xkk(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            if self.registers[vx] == kk {
                self.pc += 2;
            }
        }

        // Skip if Vx != kk
        fn op_4xkk(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            if self.registers[vx] != kk {
                self.pc += 2;
            }
        }

        // Skip if Vx == Vy
        fn op_5xy0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.registers[vx] == self.registers[vy] {
                self.pc += 2;
            }
        }

        // Set Vx = kk
        fn op_6xkk(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] = kk;
        }

        // Set Vx += kk
        fn op_7xkk(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] += kk;
        }

        // Set Vx = Vy
        fn op_8xy0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] += self.registers[vy];
        }

        // Set Vx |= Vy
        fn op_8xy1(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] |= self.registers[vy];
        }

        // Set Vx &= Vy
        fn op_8xy2(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] &= self.registers[vy];
        }

        // Set Vx ^= Vy
        fn op_8xy3(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[vx] ^= self.registers[vy];
        }

        // Set Vx += Vy, VF to borrow
        fn op_8xy4(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            let sum = vx + vy;
            self.registers[0xF] = if sum > 255 {1} else {0};
            self.registers[vx] = sum as u8 & 0xFF;

        }

        // Set Vx -= Vy, VF to NOT borrow
        fn op_8xy5(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[0xF] = (self.registers[vx] > self.registers[vy]) as u8;
            self.registers[vx] -= self.registers[vy];
        }

        // Set Vx = Vx >> 1, VF to lost bit
        fn op_8xy6(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[0xF] = self.registers[vx] & 0x1;
            self.registers[vx] >>= 1;
        }

        // Set Vx = Vy - Vx to NOT borrow
        fn op_8xy7(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            self.registers[0xF] = (self.registers[vx] < self.registers[vy]) as u8;
            self.registers[vx] = self.registers[vy] - self.registers[vx];
        }

        // Set Vx = Vx << 1, VF to lost bit
        fn op_8xye(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;
            self.registers[vx] <<= 1;
        }

        // Skip next inst if vx != vy
        fn op_9xy0(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let vy = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.registers[vx] != self.registers[vy] {
                self.pc += 2;
            }
        }

        // Set I == nnn
        fn op_annn(&mut self) {
            self.index = self.opcode & 0x0FFF;
        }

        // Jump to nnn + V0
        fn op_bnnn(&mut self) {
            self.pc += self.opcode & 0x0FFF;
        }

        // Vx = RND & kk
        fn op_cxkk(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let kk = (self.opcode & 0x00FF) as u8;
            self.registers[vx] += self.rand_num & kk;
        }

        // Display sprite at Vx, Vy, set VF = collision
        fn op_dxyn(&mut self) {
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
                        if *screen_pxl == 0xFFFFFFFF {
                            self.registers[0xF] = 1;
                        }
                        *screen_pxl ^= 0xFFFFFFFF;
                    }
                }
            }
        }

        // Skip if Vx == key
        fn op_ex9e(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            if self.keypad[self.registers[vx] as usize] != 0 {
                self.pc += 2;
            }
        }

        // Skip if Vx != key
        fn op_ex91(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            if self.keypad[self.registers[vx] as usize] == 0 {
                self.pc += 2;
            }
        }

        // Set vx to delay timer
        fn op_fx07(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.registers[vx] = self.delay_timer;
        }

        // Wait for a keypress and store the result in Vx
        fn op_fx0a(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            for i in 0..16 {
                if self.keypad[i] != 0 {
                    self.registers[vx] = i as u8;
                    return;
                }
            }
            self.pc -= 2;
        }

        // Set delay timer = Vx
        fn op_fx15(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.delay_timer = self.registers[vx];
        }

        // Set sound timer = Vx
        fn op_fx18(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.sound_timer = self.registers[vx];
        }

        // Set index += Vx
        fn op_fx1e(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.index += self.registers[vx] as u16;
        }

        // Set index += Vx
        fn op_fx29(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            self.index = FONTSET_START_ADDR as u16 + (5 * self.registers[vx] as u16);
        }

        // Store BCD representation of Vx starting from index I
        fn op_fx33(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            let mut value = self.registers[vx];

            self.memory[self.index as usize + 2] = value % 10;
            value /= 10;

            self.memory[self.index as usize + 1] = value % 10;
            value /= 10;

            self.memory[self.index as usize] = value % 10;
        }

        // Copy V0..Vx to memory
        fn op_fx55(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            for i in 0..vx {
                self.memory[self.index as usize + i] = self.registers[i];
            }
        }

        // Copy V0..Vx from memory
        fn op_fx65(&mut self) {
            let vx = ((self.opcode & 0x0F00) >> 8) as usize;
            for i in 0..vx {
                self.registers[i] = self.memory[self.index as usize + i];
            }
        }
    }
}


