use std::io::{Seek, SeekFrom, Write};

// Load from Disk Page to memory page - Page id's are not required?
pub fn load_from(&mut self, page_id_disk: usize, page_id_memory: usize, disk_page: Page) {
    self.data = disk_page.data;
    self.header = disk_page.header;
}

// From Memory Page to Disk Page: // store_to function should have write_to disk function: Inputs: File Name, page and Outputs: Result
pub fn store_to(&mut self, memory_page: Page) {
    self.header = memory_page.header;
    self.data = memory_page.data; // Here persistence code should happen - Like get relation first, then update in that relation.
}

pub fn write_to_disk(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
    file.seek(SeekFrom::Start((self.id as u64) * PAGE_SIZE as u64))?;
    file.write_all(&self.data)?;
    Ok(())
}