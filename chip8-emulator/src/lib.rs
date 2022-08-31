use rand::{Rng, thread_rng, rngs::ThreadRng};

// These are taken from Cowgod's CHIP8 specification.
const INTERPRETER_SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

const START_ADDRESS: u16 = 0x200;

const DISPLAY_MEM_WIDTH: usize = 64;
const DISPLAY_MEM_HEIGHT: usize = 32;

pub struct Chip8Processor {
    // First, we set out the things as set out in the specification
    //  --- Memory ---
    // Interpreter + working ram
    ram: [u8; 512], // A 4096 bytes ram, broken up in 8-bit (1 byte) chunks
    // Registers
    registers: [u8; 16], // 16 8-bit registers
    i_register: u16, // The 16-bit "i" register
    // Pseudo-registers
    program_counter: u16, // The pg, telling the cpu which instruction to run next
    stack: [u16; 16], // A 16-long 16-bit values stack
    stack_ptr: u8, // The stack pointer, pointing at the top of the stack

    //  --- Peripheral input ---
    keypad: [bool; 16], // The keypad is 16 hex values, 123456789ABCDEF
                        // Each input is represented here as "false" for unpressed and "true" for pressed

    //  --- Outputs ---
    display: [bool; DISPLAY_MEM_WIDTH * DISPLAY_MEM_HEIGHT],
    // The 64x32 display, represented by an array of bools. Each point is a
    // pixel, either on or off.

    //  --- Timers ---
    delay_timer: u8, // A decreasing 60Hz timer for game time
    sound_timer: u8, // A decreasing 60Hz timer for sounds
    
    // Now we add things that are not by specification, but needed in
    // our implementation of the emulator
    rng_thread: ThreadRng // An RNG thread used for random number generation
}

impl Chip8Processor {
    // The processor does 3 things: fetch, decode, execute.
    // We therefore need functions that do these three things for us.

    /// Make a new Processor, ready for execution. 
    pub fn new() -> Self {
        let mut new_processor = Self {
            ram: [0; 512], // The ram is empty
            registers: [0; 16], // The registers are empty
            i_register: 0,
            program_counter: START_ADDRESS, // Programs always start @ ram location 0x200
            stack: [0; 16], // The stack is empty
            stack_ptr: 0, // The start of the stack is at location 0
            keypad: [false; 16], // No buttons are pressed
            display: [false; DISPLAY_MEM_WIDTH * DISPLAY_MEM_HEIGHT], // The screen is completely off
            delay_timer: 0, // The timer is not set
            sound_timer: 0, // The sound timer is off
            rng_thread: thread_rng() // Make a new rng thread for random number generation
        };

        new_processor.ram[..80].copy_from_slice(&INTERPRETER_SPRITES.concat());

        new_processor
    }

    /// Push a value to the stack
    fn push(&mut self, val: u16) {
        // Protect against stack overflow
        if self.stack_ptr > self.stack.len() as u8 {
            panic!("Stack overflow!");
        }
        // Push the value where the pointer is
        self.stack[self.stack_ptr as usize] = val;
        // Point up by one.
        self.stack_ptr += 1;
    }

    /// Pop a value from the stack
    fn pop(&mut self) -> u16 {
        // Protect against a stack underflow
        if self.stack_ptr == 0 {
            panic!("Stack underflow!");
        }
        // Pop a value
        self.stack_ptr -= 1;

        self.stack[self.stack_ptr as usize]
    }

    /// Execute one Fetch-Decode-Execute cycle
    pub fn cycle(&mut self) {
        // Fetch an instruction
        let opcode = self.fetch();

        // Decode and execute the function
        self.execute(opcode);
    }

    /// Fetch the current opcode to be executed
    fn fetch(&mut self) -> u16 {
        let high_byte = self.ram[self.program_counter as usize] as u16;
        let low_byte = self.ram[(self.program_counter + 1) as usize] as u16;

        let opcode = (high_byte << 8) | low_byte;

        opcode
    }

    /// Tick the timers down by one unit (if set).
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.delay_timer > 0 {
            if self.sound_timer == 1 {
                // Code that makes it beep
            }
            self.sound_timer -= 1;
        }
    }

    /// Execute the input opcode.
    fn execute(&mut self, opcode: u16) {
        // What we do here is "OR" out the parts of the opcode that we don't
        // need, and then shift the bytes to the left, to the start of the 
        // u16. This causes the code to be left-padded by zeroes, and can
        // be interpreted directly as the new single-digit u16.
        let digits = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F
        );

        match digits {
            // 0. 0000 - NOP - Do nothing
            (0, 0, 0, 0) => return,

            // 1. 00E0 - CLS - Clear Display
            (0, 0, 0xE, 0) => {
                self.display = [false; DISPLAY_MEM_WIDTH * DISPLAY_MEM_HEIGHT]
            },

            // 2. 00EE - Return from subroutine
            (0, 0, 0xE, 0xE) => {
                let return_value = self.pop();
                self.program_counter = return_value;
            },

            // 3. 1NNN - JMP NNN - Jump to location NNN
            (1, ..) => {
                let nnn = opcode & 0xFFF;
                self.program_counter = nnn;
            },

            // 4. 2NNN - CALL NNN - Call Subroutine @NNN
            (2, ..) => {
                let nnn: u16 = opcode & 0xFFF;
                self.push(self.program_counter); // This works because u16 is Copy
                self.program_counter = nnn;
            },

            // 5. 3XNN - SKIP VX == NN - Skip ahead if
            (3, x, ..) => {
                let nn = (opcode & 0xFF) as u8;
                if self.registers[x as usize] == nn {
                    self.program_counter += 2; // 2 as we skip 2 bytes, so 1 opcode
                }
            },

            // 6. 4XNN - SKIP VX != NN - Skip ahead if not
            (4, x, ..) => {
                let nn = (opcode & 0xFF) as u8;
                if self.registers[x as usize] != nn {
                    self.program_counter += 2; // 2 as we skip 2 bytes, so 1 opcode
                }
            },

            // 7. 5XY0 - SKIP VX == VY - Skip ahead if X == Y
            (5, x, y, 0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2; // 2 as we skip 2 bytes, so 1 opcode
                }
            },
            
            // 8. 6XNN - VX = NN - Set register X to NN
            (6, x, ..) => {
                let nn = opcode & 0xFF;
                self.registers[x as usize] = nn as u8; 
            },

            // 9. 7XNN - VX + NN
            (7, x, ..) => {
                // Rust could overflow here, but Chip8 expects the numbers to wrap
                let nn = opcode & 0xFF;
                
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn as u8); 
            },

            // 10. 8XY0 - VX = VY
            (8, x, y, 0) => {
                self.registers[x as usize] = self.registers[y as usize];
            },

            // 11. 8XY1, 8XY2, 8XY3 - VX _ VY = VX, _ is OR, AND, XOR
            (8, x, y, n @ 1..=3) => {
                let (x, y) = (x as usize, y as usize);
                match n {
                    1 => self.registers[x] |= self.registers[y],
                    2 => self.registers[x] &= self.registers[y],
                    3 => self.registers[x] ^= self.registers[y],
                    _ => panic!("This is impossible to reach.")
                }
            },

            // 12. 8XY4 - ADD VX + VY  - If VX overflows, set VF to 1
            (8, x, y, 4) => {
                let (x, y) = (x as usize, y as usize);
                let (result, overflow) =
                    self.registers[x]
                    .overflowing_add(self.registers[y]);

                let overflow = if overflow {1} else {0};

                self.registers[0xF] = overflow;
                self.registers[x] = result;
            },

            // 13. 8XY5 - SUB VX - VY
            (8, x, y, 5) => {
                let (x, y) = (x as usize, y as usize);
                let (result, underflow) =
                    self.registers[x]
                    .overflowing_sub(self.registers[y]);
                
                let underflow = if underflow {0} else {1};

                self.registers[0xF] = underflow;
                self.registers[x] = result;
            },

            // 14. 8XY6 - VX >>= 1 - Bitwise shift VX by 1, and store the dropped bit in VF
            (8, x, _, 6) => {
                let x = x as usize;
                
                // The 1 here is inferred to be an u8, since it cannot be anything else.
                // 1 as u8 is 0000 0001, so we get the last digit
                let dropped = self.registers[x] & 1; 

                self.registers[x] >>= 1;
                self.registers[0xF] = dropped;
            },

            // 15. 8XY7 - SUB VY - VX  - If VX underflows, clear VF
            (8, x, y, 7) => {
                let (x, y) = (x as usize, y as usize);
                let (result, underflow) =
                    self.registers[x]
                    .overflowing_sub(self.registers[y]);
                
                let underflow = if underflow {0} else {1};

                self.registers[0xF] = underflow;
                self.registers[x] = result;
            },

            // 16. 8XY6 - VX >>= 1 - Bitwise shift VX by 1, and store the dropped bit in VF
            (8, x, _, 0xE) => {
                let x = x as usize;
                
                // Same as above, but we move the first digit to the last position,
                // so we don't have to write 1000 0000 (2^8 = 256)
                let dropped = (self.registers[x] >> 7) & 1;

                self.registers[x] <<= 1;
                self.registers[0xF] = dropped;
            },

            // 17. 9XY0 - Skip if VX != VY
            (9, x, y, 0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2; // 2 as we skip 2 bytes, so 1 opcode
                }
            },

            // 18. ANNN - Set I to 0xNNN
            (0xA, ..) => {
                let nnn: u16 = opcode & 0xFFF;

                self.i_register = nnn;
            },

            // 19. BNNN - Jump to address V0 + NNN
            (0xB, ..) => {
                let nnn: u16 = opcode & 0xFFF;
                self.program_counter = self.registers[0x0] as u16 + nnn;
            },

            // 20. CXNN - Make a random number and AND it in VX
            (0xC, x, ..) => {
                let random_num: u16 = self.rng_thread.gen();
                let nn = opcode & 0xFF;

                self.registers[x as usize] = (random_num & nn) as u8; 
            },

            // 21. DXYN - Draw n bytes from I at coordinates (VX, VY)
            // Set VF if any pixels were flipped by this action.
            (0xD, x, y, rows) => {
                let coord_x = self.registers[x as usize] as u16;
                let coord_y = self.registers[y as usize] as u16;

                let mut flipped = false;

                for y_line in 0..rows {
                    // Get the pixels we have to draw
                    let row_address = self.i_register + y_line as u16;
                    let pixels = self.ram[row_address as usize];

                    for x_line in 0..8 {
                        // We can now check for collisions and update the display
                        // Get to the pixel we are working on...
                        // We use a 1-bit mask that we move around to get
                        // the value of our pixel. If it is 1, we have to flip.
                        if (pixels & (0b10000000 >> x_line)) != 0 {
                            // The sprite can wrap the screen. so we use the modulo
                            // to go back to the beginning if we do "overflow".
                            let x = (coord_x + x_line) as usize % DISPLAY_MEM_WIDTH;
                            let y = (coord_y + y_line) as usize % DISPLAY_MEM_HEIGHT;

                            // Get the coordinate of the pixel in the screen
                            // remember that it is a 1-D array.
                            let position = x + DISPLAY_MEM_WIDTH * y;

                            flipped |= self.display[position]; // Make it true if it is not already
                            self.display[position] ^= true; // XOR on the current pixel
                        }
                    }
                }

                // If we did flip, VX has to be set to 1
                self.registers[0xF] = if flipped {1} else {0};
            },

            // 22. EX9E - Skip if the key indexed at VX is currently pressed
            (0xE, x, 9, 0xE) => {
                if self.keypad[(self.registers[x as usize]) as usize] {
                    self.program_counter += 2
                }
            },

            // 23. EXA1 - Skip if the key indexed at VX is currently unpressed
            (0xE, x, 0xA, 1) => {
                if self.keypad[(self.registers[x as usize]) as usize] {
                    self.program_counter += 2
                }
            },

            // 24. FX07 - Set VX to the delay timer
            (0xF, x, 0, 7) => {
                self.registers[x as usize] = self.delay_timer;
            },

            // 25. FX15 - Wait for any keypress. Store the keypress index in VX
            // The CPU here stops until this is the case
            (0xF, x, 0, 0xA) => {
                // I wanted to do this with a while loop, but the guide rightly 
                // suggested re-doing the instruction instead, so that the
                // `cycle` function can re-register new key presses.
                let x = x as usize;

                let mut pressed = false;

                for i in 0..self.keypad.len() {
                    if self.keypad[i] {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break
                    }
                }

                if ! pressed {
                    self.program_counter -= 2;
                }
            },

            // 26. FX15 - Set the delay timer to VX
            (0xF, x, 1, 5) => {
                self.delay_timer = self.registers[x as usize];
            },

            // 27. FX18 - Set the sound timer to VX
            (0xF, x, 1, 8) => {
                self.sound_timer = self.registers[x as usize];
            },

            // 28. FX1E - Set I to I + VX
            (0xF, x, 1, 0xE) => {
                self.i_register += self.registers[x as usize] as u16;
            },

            // 29. FX29 - Set I to the position of the interpreter font character in VX
            (0xF, x, 2, 9) => {
                // The sprites are all 5 bytes long, and start at location 0
                // in our ram. Therefore, to get their position, we multiply
                // their value (in the register) by 5, and get the corresponding
                // i_register position.
                self.i_register = (self.registers[x as usize] as u16) * 5;
            },

            // 30. FX33 - Store the BCD encoding of VX into I
            (0xF, x, 3, 3) => {
                // The BCD is a pseudo-decimal representation of a hex, stored
                // as a series of hex values. For instance, 0x64, equal to 100,
                // would become 0x1 (1), 0x0 (0), 0x0 (0), so three bytes, one
                // for each digit. As the values in our registers can go up to
                // 2^8 -1 = 255, we will always store three hex-encoded digits

                let reg_x = self.registers[x as usize] as f32;

                let hundreds = (reg_x / 100f32).floor() as u8;
                let tens = ((reg_x / 10f32) % 10f32) as u8;
                let ones = (reg_x % 10f32) as u8;

                self.ram[self.i_register as usize] = hundreds;
                self.ram[(self.i_register + 1) as usize] = tens;
                self.ram[(self.i_register + 2) as usize] = ones;
            },

            // 31. FX55 - Store V0 to VX into the RAM, starting from address I
            (0xF, x, 5, 5) => {
                for i in 0..x {
                    self.registers[i as usize] = self.ram[(self.i_register + i) as usize];
                }
            },

            // 32. FX65 - Fill V0 to VX with the RAM values starting from address I
            (0xF, x, 6, 5) => {
                for i in 0..x {
                    self.ram[(self.i_register + i) as usize] = self.registers[i as usize];
                }
            },

            // Catch-all 
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", opcode),
        }
    }

    /// Load a ROM into the RAM at the point of execution.
    pub fn load_rom(&mut self, rom:Vec<u8>) {
        // Load whatever ROM is given to us into the RAM
        for (i, opcode) in rom.into_iter().enumerate() {
            self.ram[START_ADDRESS as usize + i] = opcode;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn press_key(&mut self, key: Chip8Key) {
        let id: usize = match key {
            Chip8Key::K0 => 0,
            Chip8Key::K1 => 1,
            Chip8Key::K2 => 2,
            Chip8Key::K3 => 3,
            Chip8Key::K4 => 4,
            Chip8Key::K5 => 5,
            Chip8Key::K6 => 6,
            Chip8Key::K7 => 7,
            Chip8Key::K8 => 8,
            Chip8Key::K9 => 9,
            Chip8Key::KA => 10,
            Chip8Key::KB => 11,
            Chip8Key::KC => 12,
            Chip8Key::KD => 13,
            Chip8Key::KE => 14,
            Chip8Key::KF => 15,
        };

        self.keypad[id] = true;
    }

    pub fn release_key(&mut self, key: Chip8Key) {
        let id: usize = match key {
            Chip8Key::K0 => 0,
            Chip8Key::K1 => 1,
            Chip8Key::K2 => 2,
            Chip8Key::K3 => 3,
            Chip8Key::K4 => 4,
            Chip8Key::K5 => 5,
            Chip8Key::K6 => 6,
            Chip8Key::K7 => 7,
            Chip8Key::K8 => 8,
            Chip8Key::K9 => 9,
            Chip8Key::KA => 10,
            Chip8Key::KB => 11,
            Chip8Key::KC => 12,
            Chip8Key::KD => 13,
            Chip8Key::KE => 14,
            Chip8Key::KF => 15,
        };

        self.keypad[id] = false;
    }
}

pub enum Chip8Key {
    K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, KA, KB, KC, KD, KE, KF
}

