use std::fs::{File, remove_file};
use std::io::Read;
use storage_manager::table::{init_table, TABLE_HEADER_SIZE};

const TEST_FILE: &str = "test_table_file.bin";

#[test]
fn test_init_table() {
    // Clean up before starting
    let _ = remove_file(TEST_FILE);

    // Create a new test file
    let mut file = File::create(TEST_FILE).expect("Failed to create test file");

    // Call API
    init_table(&mut file).expect("Failed to initialize table");

    // Read back the header bytes
    let mut file = File::open(TEST_FILE).expect("Failed to reopen test file");
    let mut buffer = [1u8; TABLE_HEADER_SIZE as usize];
    file.read_exact(&mut buffer).expect("Failed to read header");

    // Check header
    assert_eq!(
        buffer,
        [0u8; TABLE_HEADER_SIZE as usize],
        "Table header should contain four 0 bytes"
    );

    // Check file size
    let metadata = file.metadata().expect("Failed to get file metadata");
    assert_eq!(
        metadata.len(),
        TABLE_HEADER_SIZE as u64,
        "File size should be exactly {} bytes (table header)",
        TABLE_HEADER_SIZE
    );
}
