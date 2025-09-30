pub const PAGE_SIZE: usize = 8192;  // Page size - storing as 8 bytes and as usize only because most pointers(file pointers and otheres requires it to be 8 bytes)

pub struct Page {
    pub data: Vec<u8>    // Actual Data
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: vec![0; PAGE_SIZE]
        }
    }
}