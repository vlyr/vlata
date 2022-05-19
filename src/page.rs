use std::path::Path;
use crate::record::ValueToType;
use std::error::Error;
use std::fs;

use crate::core::KEY_SIZE;
use crate::core::VALUE_SIZE;
use crate::record::Record;

#[derive(Debug, Clone)]
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

    /// Reads RECORD_SIZE bytes at `offset`
    pub fn read_record_at_mut_offset<T: ValueToType<T>>(&self, offset: &mut usize) -> Record<T> {
        let _value_type = self.read_byte_at_mut_offset(offset);

        let key: Vec<u8> = self.read_at_mut_offset(offset, KEY_SIZE)
            .iter()
            .filter(|x| *x != &0x00u8)
            .map(|x| *x)
            .collect();

        let value_data: Vec<u8> = self.read_at_mut_offset(offset, VALUE_SIZE)
            .iter()
            .filter(|x| *x != &0x00u8)
            .map(|x| *x)
            .collect();

        Record::from((
            String::from_utf8(key).unwrap().trim().into(),
            value_data.to_vec()
        ))
    }
}
