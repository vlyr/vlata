use crate::core::RECORD_SIZE;
use crate::record::{ValueToType, Record};
use crate::buffer::Buffer;
use std::fs;
use std::path::Path;
use std::io;

#[derive(Debug, Clone)]
pub struct Node<T> {
    records: Vec<Record<T>>
}

impl<T: ValueToType<T>> Node<T> {
    pub fn from_buffer(buffer: &Buffer) -> Self {
        let mut offset = 0;
        let _last_offset = buffer.last_idx() - (RECORD_SIZE - 1);

        let num_records = buffer.raw_data().len() / RECORD_SIZE;
        let mut records = vec![];

        for _ in 0..num_records {
            records.push(buffer.read_record_at_mut_offset(&mut offset));
        }

        Self {
            records,
        }
    }

    pub fn sort_records_by_key(&mut self) {
        self.records.sort_by_key(|record| record.key().chars().next().unwrap());
    }

    pub fn records(&self) -> &Vec<Record<T>> {
        &self.records
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.records.iter().map(|rec| rec.to_bytes()).flatten().collect()
    }

    pub fn save<S: AsRef<Path>>(&self, file_path: S) -> io::Result<()> {
        fs::write(file_path, self.as_bytes())
    }
}

