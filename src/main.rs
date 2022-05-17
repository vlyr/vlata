use std::path::Path;
use std::error::Error;
use std::fs;

const NUM_PAGES: usize = 1;

// field value type (1 byte) | key (16 bytes, UTF-8 data) | value (16 bytes) 
// field value types: 0 = u32, 1 = String
const ROW_SIZE: usize = 33;

const TYPE_SIZE: usize = 1;
const KEY_SIZE: usize = 16;
const VALUE_SIZE: usize = 16;

type Record<K, V> = (K, V);

pub struct Page {
    data: Vec<u8>
}

impl Page {
    pub fn new<T: AsRef<Path>>(data_path: T) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            data: fs::read(data_path.as_ref())?
        })
    }

    pub fn at_offset(&self, offset: usize, bytes_to_read: usize) -> &'_ [u8] {
        &self.data[offset..offset + bytes_to_read]
    }

    pub fn last_idx(&self) -> usize {
        self.data.len() - 1
    }
}

pub struct Node<V> {
    records: Vec<Record<String, V>>
}

impl<V: Ord> Node<V> {
    pub fn from_page(page: &Page) -> Self {
        let last_offset = page.last_idx() - ROW_SIZE + 1;
        let first_byte = page.at_offset(1,  1)[0];
        let last_byte = page.at_offset(last_offset, 1)[0];

        for i in first_byte..last_byte {
            println!("balls");
            println!("{}", i);
        }

        Self {
            records: vec![]
        }
    }

    pub fn sort_records(&mut self) {
        self.records.sort_by_key(|elem| elem.0.chars().next().unwrap());
    }
}

fn main() {
    let data: &mut [u8; 66] = &mut [0x00; 66];
    data[1] = 0x11;
    data[33] = 0x33;

    let page = Page::new("./example-data.buf").unwrap();
    let node: Node<()> = Node::from_page(&page);
}
