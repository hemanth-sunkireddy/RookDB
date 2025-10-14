use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::env;
use std::path::PathBuf;

use storage_manager::table::{init_table, TABLE_HEADER_SIZE};
use storage_manager::page::{page_count};

#[test]
fn test_page_count() {
    // Create a temporary file with read + write access
    let mut temp_path = PathBuf::from(env::temp_dir());
    temp_path.push("test_table_page_count.tbl");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&temp_path)
        .expect("Failed to create/open temp file");

    // Initialize table (writes 8192 bytes with page_count = 0)
    init_table(&mut file).expect("Failed to initialize table");

    // Move cursor back to start (for fresh read)
    file.seek(SeekFrom::Start(0)).unwrap();

    // Call page_count() to read first page and extract page count
    let count = page_count(&mut file).expect("Failed to read page count");

    // Verify the page count is 0
    assert_eq!(count, 0, "Expected page count to be 0 after initialization");

    // Verify the file size equals TABLE_HEADER_SIZE (8192 bytes)
    let file_size = file.metadata().unwrap().len();
    assert_eq!(
        file_size,
        TABLE_HEADER_SIZE as u64,
        "File size should equal TABLE_HEADER_SIZE (8192 bytes)"
    );
}
