pub const RECORD_SIZE: usize = 32;

pub const KEY_SIZE: usize = 16;
pub const VALUE_SIZE: usize = 16;

pub fn generate_padding_bytes(data: &[u8]) -> Vec<u8> {
    // key & value size are the same, will be changed later
    let len = KEY_SIZE - data.len();
    vec![0x0; len]
}
