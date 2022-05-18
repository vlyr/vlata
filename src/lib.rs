// field value type (1 byte) | key (15 bytes, UTF-8 data) | value (16 bytes) 
// field value types: 0 = u32, 1 = String

pub mod page;
pub use crate::page::Page;

pub mod core;

pub mod node;
pub use crate::node::Node;

pub mod row;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row::ValueType;

    use std::fs;

    const EXAMPLE_KEY: &[u8] = b"hello";
    const EXAMPLE_VALUE: &[u8] = b"world";
    const KEY_PADDING_DATA_LEN: usize = core::KEY_SIZE - EXAMPLE_KEY.len();
    const VALUE_PADDING_DATA_LEN: usize = core::VALUE_SIZE - EXAMPLE_VALUE.len();

    #[test]
    fn rw_page_data() {
        let data: &mut [u8; 66] = &mut [0x00; 66];

        data[0] = 0x01;
        data[1..=15].clone_from_slice(&[EXAMPLE_KEY, &[0u8; KEY_PADDING_DATA_LEN]].concat());
        data[16..=31].clone_from_slice(&[EXAMPLE_VALUE, &[0u8; VALUE_PADDING_DATA_LEN]].concat());

        data[33] = 0x01;
        data[34..=48].clone_from_slice(&[EXAMPLE_KEY, &[0u8; KEY_PADDING_DATA_LEN]].concat());
        data[49..=64].clone_from_slice(&[EXAMPLE_VALUE, &[0u8; VALUE_PADDING_DATA_LEN]].concat());

        fs::write("./example-data.buf", data).unwrap();

        let page = Page::new("./example-data.buf").unwrap();

        let mut offset = 0;
        let row = page.read_row_at_mut_offset(&mut offset);

        assert_eq!(row.key().as_str(), "hello");
        assert_eq!(row.value().as_str(), "world");
        assert_eq!(row.value_type(), &ValueType::Str)
    }
}
