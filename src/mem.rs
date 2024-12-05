type Byte = u8;


const MAX_MEM: u32 = 1024 * 64;

/* 
 * Memory
 */ 
pub struct Memory {
    pub data: Vec<Byte>
}

impl Default for Memory{
    fn default() -> Self {
        Memory { data: vec![0; MAX_MEM as usize] }
    }
}

impl Memory {
    pub fn restart(&mut self) {
        for i in self.data.iter_mut() {
            *i = 0;
        }
    }

    pub fn get_byte(&mut self, address: u32) -> Byte{
        let option= self.data.get(address as usize);

        if option.is_none() {
            println!("Error");
            return 0x0000
        }

        *option.unwrap()
    }
}