
/*
 * CPU
 */

type Byte = u8;
type Word = u16;

use crate::mem;

const NEG_MASK: Byte = 0b10000000;

pub const INS_ADC_IM:  u8 = 0x69;
pub const INS_BVS_REL: u8 = 0x70;
pub const INS_ADC_INY: u8 = 0x71;
pub const INS_ADC_ZPX: u8 = 0x75;
pub const INS_ROR_ZPX: u8 = 0x76;
pub const INS_SEI_IMP: u8 = 0x78;
pub const INS_ADC_ABY: u8 = 0x79;
pub const INS_ADC_ABX: u8 = 0x7D;
pub const INS_ROR_ABX: u8 = 0x7E;
pub const INS_STA_INX: u8 = 0x81;
pub const INS_STY_ZP:  u8 = 0x84;
pub const INS_STA_ZP:  u8 = 0x85;
pub const INS_STX_ZP:  u8 = 0x86;
pub const INS_DEY_IMP: u8 = 0x88;
pub const INS_TXA_IMP: u8 = 0x8A;
pub const INS_STY_ABS: u8 = 0x8C;
pub const INS_STA_ABS: u8 = 0x8D;
pub const INS_STX_ABS: u8 = 0x8E;
pub const INS_BCC_REL: u8 = 0x90;
pub const INS_STA_INY: u8 = 0x91;
pub const INS_STY_ZPX: u8 = 0x94;
pub const INS_STA_ZPX: u8 = 0x95;
pub const INS_STX_ZPY: u8 = 0x96;
pub const INS_TYA_IMP: u8 = 0x98;
pub const INS_STA_ABY: u8 = 0x99;
pub const INS_TXS_IMP: u8 = 0x9A;
pub const INS_STA_ABX: u8 = 0x9D;
pub const INS_LDY_IM:  u8 = 0xA0;
pub const INS_LDA_IDX: u8 = 0xA1;
pub const INS_LDX_IM:  u8 = 0xA2;
pub const INS_LDY_ZP:  u8 = 0xA4;
pub const INS_LDA_ZP:  u8 = 0xA5;
pub const INS_LDX_ZP:  u8 = 0xA6;
pub const INS_TAY_IMP: u8 = 0xA8;
pub const INS_LDA_IM:  u8 = 0xA9;
pub const INS_BCS_REL: u8 = 0xB0;
pub const INS_LDA_INY: u8 = 0xB1;
pub const INS_LDY_ZPX: u8 = 0xB4;
pub const INS_LDA_ZPX: u8 = 0xB5;
pub const INS_LDX_ZPY: u8 = 0xB6;
pub const INS_CLV_IMP: u8 = 0xB8;
pub const INS_LDA_ABY: u8 = 0xB9;
pub const INS_TSX_IMP: u8 = 0xBA;


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
            let instruction = self.fetch(&mut cycles, mem);
            CPU::handle_instruction(self, instruction, &mut cycles, mem);
            
        }
    }

    fn handle_instruction(&mut self, instruction: u8, cycles: &mut u32, mem: &mut mem::Memory){
        match instruction {

            // 0x69
            INS_ADC_IM => {
                let value: Byte = self.fetch(cycles, mem);
                
                let result: Word = (self.a + value + self.c).into();
                self.c = if result > 0xFF {1} else {0};
                self.z = if result == 0 {1} else {0};
                
            },


            INS_LDA_IM => {
                
                let value: Byte = self.fetch(cycles, mem);
                
                self.a = value;
                self.z = if self.a == 0  {1} else { 0 };
                self.n = if (self.a & NEG_MASK) > 0 {1} else {0};
                
            },

            _ => {
                println!("Instruction {} not handled!", instruction);
            }
        }
    }
}

