use std::io::{Seek, SeekFrom, Write};
pub const PAGE_SIZE: usize = 8192;  // Page size
pub const PAGE_HEADER_SIZE: usize = 400; // Page Header Size

pub struct PageHeader {
    // pub page_no: usize,   // Page number or page id?
    pub lower: usize,
    pub upper: usize,
    pub magic: usize
}
pub struct Page {
    pub header: PageHeader,
    pub id: usize,                // 4 bytes integer - 4 billion page id's are possible.
    pub data: [u8; PAGE_SIZE]   // Array of page size elements
}

impl PageHeader {
    pub fn new(id: usize, magic_no: usize) -> Self {
        Self {
            // page_no: id,
            lower: 0,
            upper: PAGE_HEADER_SIZE,
            magic: magic_no
        }
    }
}

impl Page {
    pub fn new(id: usize) -> Self {
        Self { header: PageHeader::new(1, 32 ), id, data: [0; PAGE_SIZE]
        }
    }

    // Load from Disk Page to memory page - Page id's are not required?
    pub fn load_from(&mut self, page_id_disk: usize, page_id_memory: usize, disk_page: Page) {
        self.data = disk_page.data;
        self.header = disk_page.header;
    }

    // From Memory Page to Disk Page: // store_to function should have write_to disk function: Inputs: File Name, page and Outputs: Result
    pub fn store_to(&mut self, memory_page: Page) {
        self.header = memory_page.header;
        self.data = memory_page.data;   // Here persistence code should happen - Like get relation first, then update in that relation.
    }

    pub fn write_to_disk(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.seek(SeekFrom::Start((self.id as u64) * PAGE_SIZE as u64))?;
        file.write_all(&self.data)?;
        Ok(())
    }
}