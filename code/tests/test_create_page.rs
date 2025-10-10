use std::fs::{File, remove_file};
use storage_manager::disk::create_page;
use storage_manager::page::PAGE_SIZE;

const TEST_FILE: &str = "test_page_file.bin";

#[test]
fn test_create_page() {
    // Clean up before starting
    let _ = remove_file(TEST_FILE);

    // Create a new test file
    let mut file = File::create(TEST_FILE).expect("Failed to create test file");

    // Call API
    let _ = create_page(&mut file);

    // Check that the file size == PAGE_SIZE
    let metadata = file.metadata().expect("Failed to get file metadata");
    assert_eq!(
        metadata.len(),
        PAGE_SIZE as u64,
        "File size should be exactly one page ({} bytes)",
        PAGE_SIZE
    );
}
