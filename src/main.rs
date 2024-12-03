

pub mod cpu;
pub mod mem;



fn main() {
    let mut cpu: cpu::CPU = cpu::CPU {..Default::default()};
    let mut mem: mem::Memory = mem::Memory {..Default::default()};
    cpu.reset(&mut mem);

    cpu.execute(2, &mut mem);
}


