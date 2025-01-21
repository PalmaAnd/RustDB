use log::info;
use tempfile::NamedTempFile;

mod sql {
    pub mod abstract_syntax;
}
mod buffer;
mod disk;
mod error;
mod page;
mod record;
mod table;

use crate::buffer::BufferPoolManager;
use crate::disk::DiskManager;
use crate::error::Result;
use crate::table::Table;

fn test_thus() -> Result<()> {
    // Create a temp db file
    let temp_file = NamedTempFile::new()?;
    info!("Created temporary database at: {:?}", temp_file.path());

    // Init disk manager
    let disk_manager = DiskManager::new(temp_file.path())?;

    // Init buffer pool manager
    let buffer_pool = BufferPoolManager::new(disk_manager);

    // Create a table
    let mut table = Table::new("users".to_string(), buffer_pool);
    Ok(())
}

fn main() {
    // Initialize logging
    env_logger::init();
    info!("RustDB starting...");

    match test_thus() {
        Ok(()) => info!("Test good!"),
        Err(e) => eprintln!("Error running test: {:?}", e),
    }
}
