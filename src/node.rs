use crate::core::{Record, ROW_SIZE};
use crate::page::Page;

#[derive(Debug, Clone)]
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

