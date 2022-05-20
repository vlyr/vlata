use crate::core::RECORD_SIZE;
use crate::record::{ValueToType, Record};
use crate::buffer::Buffer;
use std::fs;
use std::path::Path;
use std::io;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Node<T> {
    records: Vec<Record<T>>,
    buffer: Buffer,
}

impl<T: ValueToType<T>> Node<T> {
    pub fn new<P: AsRef<Path>>(data_path: P) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::new(data_path)?;

        let mut offset = 0;
        let _last_offset = buffer.last_idx() - (RECORD_SIZE - 1);

        let num_records = buffer.raw_data().len() / RECORD_SIZE;
        let mut records = vec![];

        for _ in 0..num_records {
            records.push(buffer.read_record_at_mut_offset::<T>(&mut offset));
        }

        Ok(Self {
            buffer,
            records,
        })
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

    pub fn insert<S: AsRef<str>>(&mut self, key: S, value: T) {
        let record = Record::from((key.as_ref().to_string(), T::to_bytes(value)));

        self.records.push(record)
    }
}

