// field value type (1 byte) | key (15 bytes, UTF-8 data) | value (16 bytes) 
// field value types: 0 = u32, 1 = String

pub mod buffer;
pub use crate::buffer::Buffer;

pub mod core;

pub mod node;
pub use crate::node::Node;

pub mod record;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::Record;
    use crate::record::ValueToType;
    use std::fs;

    const EXAMPLE_KEY: &[u8] = b"hello";
    const EXAMPLE_VALUE: &[u8] = b"world";
    const KEY_PADDING_DATA_LEN: usize = core::KEY_SIZE - EXAMPLE_KEY.len();
    const VALUE_PADDING_DATA_LEN: usize = core::VALUE_SIZE - EXAMPLE_VALUE.len();

    fn print_node_records<T: ValueToType<T> + Clone + std::fmt::Display>(node: &Node<T>) {
        let records: Vec<String> = node.records()
            .iter()
            .map(|rec| format!("{} -> {}", rec.key(), rec.value()))
            .collect();

        println!("{}", records.join("\n"));
    }

    fn generate_example_data() -> [u8; 64] {
        let data: &mut [u8; 64] = &mut [0x00; 64];

        data[0..16].clone_from_slice(&[
            EXAMPLE_KEY,
            &[0u8; KEY_PADDING_DATA_LEN]
        ].concat());

        data[16..32].clone_from_slice(&[
            EXAMPLE_VALUE,
            &[0u8; VALUE_PADDING_DATA_LEN]
        ].concat());

        data[32..48].clone_from_slice(&[
            EXAMPLE_KEY,
            &[0u8; KEY_PADDING_DATA_LEN]
        ].concat());

        data[48..64].clone_from_slice(&[
            EXAMPLE_VALUE,
            &[0u8; VALUE_PADDING_DATA_LEN]
        ].concat());

        data.clone()
    }

    #[test]
    fn rw_buffer_data() {
        //println!("{:#?}", data);

        fs::write("./example-data.buf", generate_example_data()).unwrap();

        let buffer = Buffer::new("./example-data.buf").unwrap();

        let mut offset = 0;
        let record: Record<String> = buffer.read_record_at_mut_offset(&mut offset);

        assert_eq!(record.key().as_str(), "hello");
        assert_eq!(record.value(), "world");
    }

    #[test]
    fn node() {
        let node: Node<String> = Node::new("./example-data.buf").unwrap();


        print_node_records(&node);
        //assert_eq!(node.as_bytes(), generate_example_data());
    }

    #[test]
    fn node_save() {
        let node: Node<String> = Node::new("./example-data.buf").unwrap();
        
        node.save("./example-data.buf").unwrap();
    }

    #[test]
    fn node_insert() {
        let mut node: Node<String> = Node::new("./example-data.buf").unwrap();
        
        node.insert("hello", "world".into());
        node.sort_records_by_key();

        print_node_records(&node);

        node.save("./example-data.buf").unwrap();
    }

    #[test]
    fn node_delete() {
        let mut node: Node<String> = Node::new("./example-data.buf").unwrap();
        
        node.delete("hello");

        let records: Vec<String> = node.records()
            .iter()
            .map(|rec| format!("{} -> {}", rec.key(), rec.value()))
            .collect();

        println!("{}", records.join("\n"));

        node.save("./example-data.buf").unwrap();
    }
}
