/*
For more informations:
https://www.nesdev.org/wiki/CPU_ALL

opcodes https://www.nesdev.org/wiki/CPU_unofficial_opcodes

The status flags:
7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No CPU effect, see: the B flag
|+-------- Overflow
+--------- Negative
 */

pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s_flags: u8,
    pub memory: [u8; 0xFFFF],
}

// Implementation block, all `COU` associated functions & methods go in here
impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s_flags: 0,
            memory: [],
        }
    }

    pub fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize];
    }

    pub fn write_memory(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn interpreter(&mut self, instructions: Vec<u8>) {
        loop {
            let opcodes = self.read_memory(self.pc);
            self.pc += 1;
            match opcodes {
                // 0xA9 is where we start reading in data!
                0xA9 => {
                    let i_pc = instructions[self.pc as usize];
                    self.pc += 1;
                    self.a = i_pc;
                    if self.a == 0 {
                        self.s_flags = self.s_flags | 0b0000_0010; // Set the seventh bit to one which is Z NVss_DIZC
                    } else {
                        self.s_flags = self.s_flags & 0b1111_1101; // Will set the flags as they are
                    }
                    if self.a & 0b1000_000 != 0 {
                        self.s_flags = self.s_flags | 0b1000_0000;
                    } else {
                        self.s_flags = self.s_flags & 0b0111_1111;
                    }
                }

                // 0xAA TAX - Transfer Accumulator to X
                0xAA => {
                    self.x = self.a;

                    if self.x == 0 {
                        self.s_flags = self.s_flags | 0b0000_0010;
                    } else {
                        self.s_flags = self.s_flags & 0b1111_1101;
                    }
                    if self.x & 0b1000_000 != 0 {
                        self.s_flags = self.s_flags | 0b1000_0000;
                    } else {
                        self.s_flags = self.s_flags & 0b0111_1111;
                    }
                }

                // 0x00 is the break, that is where we stop
                0x00 => {
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn load_and_interpret(&mut self, program: Vec<u8>) {
        self.load(program);
        self.interpreter()
    }
}

fn main() {
    println!("Hello, CPU!");
}
