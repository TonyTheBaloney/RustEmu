/**
 * Stack
 */

type Byte = u8;

pub struct Stack {
    pub data: Vec<Byte>
}

const MAX_STACK_SIZE: u32 = 256;

impl Default for Stack{
    fn default() -> Self {
        Stack { data: vec![0; MAX_STACK_SIZE as usize] }
    }
}



