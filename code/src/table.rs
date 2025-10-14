use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

pub const TABLE_HEADER_SIZE: u32 = 8192;

pub struct Table {
    pub data: Vec<u8>, // Fixed-size buffer holds the raw bytes of a table.
}

pub struct TableHeader {
    pub page_count: u32, // Total Number of Pages in a Table
}

// pub struct Table {
//     pub table_header: TableHeader;
// }


impl Table {
    pub fn new() -> Self {
        Self {
            data: vec![0; TABLE_HEADER_SIZE as usize], // 8192 bytes initialized to 0 in Memory
        }
    }
}

pub fn init_table(file: &mut File) -> io::Result<()> {
    // Move cursor to the beginning of the file
    file.seek(SeekFrom::Start(0))?;

    // Write 8192 bytes of zeros to the file
    let zero_buf = vec![0u8; TABLE_HEADER_SIZE as usize];
    file.write_all(&zero_buf)?;

    Ok(())
}
