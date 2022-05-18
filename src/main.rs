use std::path::Path;
use std::error::Error;
use std::fs;

const NUM_PAGES: usize = 1;

const EXAMPLE_KEY: &[u8] = b"hello";
const EXAMPLE_VALUE: &[u8] = b"world";
const JUNK_DATA_LEN: usize = KEY_SIZE - EXAMPLE_KEY.len();

// field value type (1 byte) | key (16 bytes, UTF-8 data) | value (16 bytes) 
// field value types: 0 = u32, 1 = String
const ROW_SIZE: usize = 33;

const TYPE_SIZE: usize = 1;
const KEY_SIZE: usize = 16;
const VALUE_SIZE: usize = 16;

type Record<K, V> = (K, V);

pub enum ValueType {
    Str,
    Uint
}

type Row<V> = (ValueType, String, V);

pub struct Page {
    data: Vec<u8>
}

impl Page {
    pub fn new<T: AsRef<Path>>(data_path: T) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            data: fs::read(data_path.as_ref())?
        })
    }

    pub fn read_at_mut_offset(&self, offset: &mut usize, bytes_to_read: usize) -> &'_ [u8] {
        *offset += bytes_to_read;
        &self.data[*offset - bytes_to_read..*offset]
    }

    pub fn read_at_offset(&self, offset: usize, bytes_to_read: usize) -> &'_ [u8] {
        &self.data[offset..offset + bytes_to_read]
    }

    pub fn read_byte_at_mut_offset(&self, offset: &mut usize) -> u8 {
        *offset += 1;
        self.data[*offset - 1]
    }

    pub fn read_byte_at_offset(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn last_idx(&self) -> usize {
        self.data.len() - 1
    }
    
    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn read_row_at_mut_offset(&self, offset: &mut usize) -> Row<String> {
        let value_type = self.read_byte_at_mut_offset(offset);

        println!("{}", value_type);

        let key = self.read_at_mut_offset(offset, KEY_SIZE);
        let value = self.read_at_mut_offset(offset, VALUE_SIZE);

        println!("type={}, key={}, value={}", value_type, String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());

        (ValueType::Str, String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap())
    }
}

pub struct Node<V> {
    records: Vec<Record<String, V>>
}

impl<V: Ord> Node<V> {
    pub fn from_page(page: &Page) -> Self {
        let mut offset = 0;
        let last_offset = page.last_idx() - (ROW_SIZE - 1);

        println!("{:#?}", page.raw_data());

        page.read_row_at_mut_offset(&mut offset);
        page.read_row_at_mut_offset(&mut offset);

        Self {
            records: vec![]
        }
    }

    pub fn sort_records_by_key(&mut self) {
        self.records.sort_by_key(|elem| elem.0.chars().next().unwrap());
    }
}

fn main() {
    let data: &mut [u8; 66] = &mut [0x00; 66];

    data[0] = 0x01;
    data[1..=16].clone_from_slice(&[EXAMPLE_KEY, &[0u8; JUNK_DATA_LEN]].concat());
    data[17..=32].clone_from_slice(&[EXAMPLE_VALUE, &[0u8; JUNK_DATA_LEN]].concat());

    data[33] = 0x01;
    data[34..=49].clone_from_slice(&[EXAMPLE_KEY, &[0u8; JUNK_DATA_LEN]].concat());
    data[50..=65].clone_from_slice(&[EXAMPLE_VALUE, &[0u8; JUNK_DATA_LEN]].concat());

    fs::write("./example-data.buf", data).unwrap();

    let page = Page::new("./example-data.buf").unwrap();
    let node: Node<()> = Node::from_page(&page);
}
