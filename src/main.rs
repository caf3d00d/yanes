/*
For more informations:
https://www.nesdev.org/wiki/CPU_ALL

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
    pub a:	u8,
	pub x:	u8,
	pub y:	u8,
	pub pc:	u16,
	pub s_flags:	u8,
}

// Implementation block, all `COU` associated functions & methods go in here
impl CPU {
	pub fn new() -> Self {
		CPU {
			a: 0,
			x: 0,
			y: 0,
			pc: 0,
			s_flags: 0
		}
	}

	pub fn interpreter(&mut self, instructions: Vec<u8>) {
		//init();

		loop {
			// An opcode should be a byte size so I use as usize
			let opcodes = instructions[self.pc as usize];
			// Noice job rust -> https://github.com/dtolnay/rust-faq#why-doesnt-rust-have-increment-and-decrement-operators
			self.pc += 1;
			match opcodes {
				// opcodes https://www.nesdev.org/wiki/CPU_unofficial_opcodes
				// 0xA9 is where we start reading in data! Basically,
				// the byte that immediately follows the opcode in memory is our “target”.
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
}

fn main() {
    println!("Hello, CPU!");
}
