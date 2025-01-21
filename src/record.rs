use serde::{Deserialize, Serialize};

use crate::error::{DBError, Result};
use crate::page::Page;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub data: Vec<u8>,
}

impl Record {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone)]
pub struct RecordId {
    pub page_id: u32,
    pub slot_id: u16,
}

pub struct PageSlotIterator<'a> {
    page: &'a Page,
    current_slot: u16,
    slot_count: u16,
}

impl<'a> Iterator for PageSlotIterator<'a> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {}
}

pub struct SlottedPage;

impl SlottedPage {
    const HEADER_SIZE: usize = 4;
    const SLOT_SIZE: usize = 4; // 2 bytes offset + 2 bytes length

    pub fn init(page: &mut Page) {}

    pub fn insert_record(page: &mut Page, record: &Record) -> Result<u16> {}

    pub fn get_record(page: &Page, slot_id: u16) -> Result<Record> {}

    pub fn iter(page: &Page) -> PageSlotIterator {}
}
