use std::fs::File;
use std::path::Path;

use crate::error::Result;
use crate::page::Page;

pub struct DiskManager {
    heap_file: File,
    next_page_id: u32,
}

impl DiskManager {
    /// Creates a new DiskManager with a new database file
    pub fn new(heap_file_path: impl AsRef<Path>) -> Result<Self> {}

    /// Opens an existing database file
    pub fn open(heap_file_path: impl AsRef<Path>) -> Result<Self> {}

    /// Allocates a new page and returns its page_id
    pub fn allocate_page(&mut self) -> Result<u32> {}

    /// Reads a page from disk
    pub fn read_page(&mut self, page_id: u32) -> Result<Page> {}

    /// Writes a page to disk
    pub fn write_page(&mut self, page: &Page) -> Result<()> {}

    /// Calculates the offset of a page in the file
    fn page_offset(&self, page_id: u32) -> u64 {}
}
