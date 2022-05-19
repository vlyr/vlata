// field value type (1 byte) | key (15 bytes, UTF-8 data) | value (16 bytes) 
// field value types: 0 = u32, 1 = String

pub mod page;
pub use crate::page::Page;

pub mod core;

pub mod node;
pub use crate::node::Node;

pub mod record;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::Record;

    use std::fs;

    const EXAMPLE_KEY: &[u8] = b"hello";
    const EXAMPLE_VALUE: &[u8] = b"world";
    const KEY_PADDING_DATA_LEN: usize = core::KEY_SIZE - EXAMPLE_KEY.len();
    const VALUE_PADDING_DATA_LEN: usize = core::VALUE_SIZE - EXAMPLE_VALUE.len();

    fn generate_example_data() -> [u8; 64] {
        let data: &mut [u8; 64] = &mut [0x00; 64];

        data[0] = 0x01;
        data[1..=15].clone_from_slice(&[
            EXAMPLE_KEY,
            &[0u8; KEY_PADDING_DATA_LEN]
        ].concat());

        data[16..=31].clone_from_slice(&[
            EXAMPLE_VALUE,
            &[0u8; VALUE_PADDING_DATA_LEN]
        ].concat());

        data[32] = 0x01;

        data[33..=47].clone_from_slice(&[
            EXAMPLE_KEY,
            &[0u8; KEY_PADDING_DATA_LEN]
        ].concat());

        data[48..=63].clone_from_slice(&[
            EXAMPLE_VALUE,
            &[0u8; VALUE_PADDING_DATA_LEN]
        ].concat());

        data.clone()
    }

    #[test]
    fn rw_page_data() {
        //println!("{:#?}", data);

        fs::write("./example-data.buf", generate_example_data()).unwrap();

        let page = Page::new("./example-data.buf").unwrap();

        let mut offset = 0;
        let record: Record<String> = page.read_record_at_mut_offset(&mut offset);

        assert_eq!(record.key().as_str(), "hello");
        assert_eq!(record.value(), "world");
    }

    #[test]
    fn node() {
        fs::write("./example-data.buf", generate_example_data()).unwrap();

        let page = Page::new("./example-data.buf").unwrap();

        let node: Node<String> = Node::from_page(&page);

        let records: Vec<String> = node.records()
            .iter()
            .map(|rec| format!("{} -> {}", rec.key(), rec.value()))
            .collect();

        println!("{}", records.join("\n"));
    }
}
