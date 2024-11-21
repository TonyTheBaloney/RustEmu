use std::{mem, usize};

type Byte = u8;
type Word = u16;

const MAX_MEM: u32 = 1024 * 64;


fn main() {
    let mut cpu: CPU = CPU {..Default::default()};
    let mut mem: Memory = Memory {..Default::default()};
    cpu.reset(&mut mem);

    cpu.execute(2, &mut mem);
}


struct CPU {
    pc: Word, // Program Counter
    sp: Byte, // Stack Pointer
    
    a: Byte, // A Register
    x: Byte, // X Register
    y: Byte, // Y Register

    c: Byte, // Carry               Status Flag
    z: Byte, // Zero                Status Flag
    i: Byte, // Interrupt Distable  Status Flag
    d: Byte, // Decimal             Status Flag
    b: Byte, // B                   Flag
    v: Byte, // Overflow            Status Flag
    n: Byte, // Negative            Status Flag
}

impl Default for CPU {
    fn default() -> Self {
        CPU { pc: 0, sp: 0, a: 0, x: 0, y: 0, c: 0, z: 0, i: 0, d: 0, b: 0, v: 0, n: 0 }
    }
}

impl CPU {
    fn reset(&mut self, mem: &mut Memory) {
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

    fn fetch(&mut self, cycles: &mut u32, mem: &mut Memory) -> Byte{
        let idx = usize::try_from(self.pc).unwrap();
        let data: Byte = mem.data[idx];
        self.pc += 1;
        *cycles -= 1;
        data
    }

    fn execute(&mut self, mut cycles: u32, mem: &mut Memory){
        while cycles > 0 {

            let instruction = self.fetch( &mut cycles, mem);
            

        }
    }
}


struct Memory {
    data: Vec<Byte>
}

impl Default for Memory{
    fn default() -> Self {
        Memory { data: Vec::with_capacity( usize::try_from(MAX_MEM).unwrap() ) }
    }
}

impl Memory {
    fn restart(&mut self) {
        for i in self.data.iter_mut() {
            *i = 0;
        }
    }
}