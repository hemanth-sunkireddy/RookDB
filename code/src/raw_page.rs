pub const PAGE_SIZE: usize = 8192;  // Page size - storing as 8 bytes and as usize only because most pointers(file pointers and otheres requires it to be 8 bytes)

pub struct Page {
    pub data: Vec<u8>  // Fixed-size buffer holds the raw bytes of a page (PAGE_SIZE = 8KB)
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: vec![0; PAGE_SIZE]
        }
    }
}