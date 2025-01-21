use crate::error::{DBError, Result};
use serde::{Deserialize, Serialize};

pub const PAGE_SIZE: usize = 4096; // Default size defined by SQLLite

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page_id: u32,
    pub data: Vec<u8>,
}

/// Represents a page in the database with a fixed size.
///
/// # Fields
/// - `page_id`: The unique identifier for the page.
/// - `data`: The actual data stored in the page, represented as a vector of bytes.
///
/// # Methods
/// - `new(page_id: u32) -> Self`: Creates a new page with the given `page_id` and initializes the data to a vector of zeros with a size of `PAGE_SIZE`.
/// - `get_data(&self) -> &[u8]`: Returns a reference to the data stored in the page.
/// - `write_data(&mut self, offset: usize, data: &[u8]) -> Result<()>`: Writes the given `data` to the page starting at the specified `offset`. Returns an error if the data exceeds the page size.
impl Page {
    pub fn new(page_id: u32) -> Self {
        Self {
            page_id,
            data: vec![0; PAGE_SIZE],
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn write_data(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        if offset + data.len() > PAGE_SIZE {
            return Err(DBError::Page("Data exceeds page size".to_string()));
        }
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}
