
/*
 * CPU
 */

type Byte = u8;
type Word = u16;

use crate::mem;

pub const INS_LDA_IM: u8 = 0xA9;


pub struct CPU {
    pub pc: Word, // Program Counter
    pub sp: Byte, // Stack Pointer
    
    pub a: Byte, // A Register
    pub x: Byte, // X Register
    pub y: Byte, // Y Register

    pub c: Byte, // Carry               Status Flag
    pub z: Byte, // Zero                Status Flag
    pub i: Byte, // Interrupt Distable  Status Flag
    pub d: Byte, // Decimal             Status Flag
    pub b: Byte, // B                   Flag
    pub v: Byte, // Overflow            Status Flag
    pub n: Byte, // Negative            Status Flag
}

impl Default for CPU {
    fn default() -> Self {
        CPU { pc: 0, sp: 0, a: 0, x: 0, y: 0, c: 0, z: 0, i: 0, d: 0, b: 0, v: 0, n: 0 }
    }
}

impl CPU {
    pub fn reset(&mut self, mem: &mut mem::Memory) {
        self.pc = 0xFFFC;
        self.sp = 0x10;
        
        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        mem.restart();
    }

    pub fn fetch(&mut self, cycles: &mut u32, mem: &mut mem::Memory) -> Byte{
        let data: Byte = mem.get_byte(self.pc as u32);
        self.pc += 1;
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, mut cycles: u32, mem: &mut mem::Memory){
        while cycles > 0 {

            let instruction = self.fetch( &mut cycles, mem);
            CPU::handle_instruction(self, instruction, cycles, mem);

        }
    }

    fn handle_instruction(&mut self, instruction: u8, mut cycles: u32, mem: &mut mem::Memory){
        match instruction {
            INS_LDA_IM => {
                let value: Byte = self.fetch(&mut cycles, mem);
                self.a = value;
                self.z = if self.a == 0  {1 } else { 0 };
                self.n = if (self.a & 0b10000000) > 0 {1} else {0};
                
            },
            _ => {
                println!("Instruction not handled!");
            }
        }
    }
}

