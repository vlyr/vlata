use crate::core::RECORD_SIZE;
use crate::record::{ValueToType, Record};
use crate::page::Page;

#[derive(Debug, Clone)]
pub struct Node<T> {
    records: Vec<Record<T>>
}

impl<T: ValueToType<T>> Node<T> {
    pub fn from_page(page: &Page) -> Self {
        let mut offset = 0;
        let _last_offset = page.last_idx() - (RECORD_SIZE - 1);

        let num_records = page.raw_data().len() / RECORD_SIZE;
        let mut records = vec![];

        for _ in 0..num_records {
            records.push(page.read_record_at_mut_offset(&mut offset));
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
}

