use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read, ErrorKind, Error, Write};

use crate::raw_page::{Page, PAGE_SIZE};

// Create Page 
pub fn create_page(file: &mut File) -> io::Result<u32> {
    // Create an empty page (all zeros)
    let page = Page::new();

    // Get current file size
    let file_size = file.metadata()?.len();

    // Calculate the page number for the new page
    let page_num = (file_size / PAGE_SIZE as u64) as u32;

    // Move cursor to end of file
    file.seek(SeekFrom::End(0))?;

    // Write the zero-filled page
    file.write_all(&page.data)?;

    println!("Created new page {} at offset {}", page_num, file_size);

    Ok(page_num) // return the page number of the newly created page
}

// Read page from disk
pub fn read_page(file: &mut File, page: &mut Page, page_num: u32) -> io::Result<()> {   // Page Number or Page Id - as offset. (For Contiguous - PageNum * offset is ok but pageId requires more)
    // calculating the offset
    let offset: u64 = (page_num as u64) * (PAGE_SIZE as u64);   // as is required because not compiling - pageNum is 4 byte but offset requries 8 bytes.

    // get file size
    let file_size = file.metadata()?.len();
    println!("File Size: {}", file_size);

    if offset > file_size {
        // Return an error if the page doesn't exist
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("Page {} does not exist in the file", page_num),
        ));
    }

    // move the file cursor
    file.seek(SeekFrom::Start(offset))?;

    // read the page data
    file.read_exact(&mut page.data)?;

    Ok(())
}

// Write Page into Disk
pub fn write_page(file: &mut File, page: &mut Page, page_num: u32) -> io::Result<()> {   // Page Number or Page Id - as offset. (For Contiguous - PageNum * offset is ok but pageId requires more)
    // calculating the offset
    let offset: u64 = (page_num as u64) * (PAGE_SIZE as u64);   // as is required because not compiling - pageNum is 4 byte but offset requries 8 bytes.

    // get file size
    let file_size = file.metadata()?.len();
    println!("File Size: {}", file_size);

    if offset > file_size {
        // Return an error if the page doesn't exist
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("Page {} does not exist in the file", page_num),
        ));
    }

    // move the file cursor
    file.seek(SeekFrom::Start(offset))?;

    // Write the page data into the file at that offset
    file.write_all(&page.data)?;

    Ok(())
}

// // Load from Disk Page to memory page - Page id's are not required?
// pub fn load_from(&mut self, page_id_disk: usize, page_id_memory: usize, disk_page: Page) {
//     self.data = disk_page.data;
//     self.header = disk_page.header;
// }

// // From Memory Page to Disk Page: // store_to function should have write_to disk function: Inputs: File Name, page and Outputs: Result
// pub fn store_to(&mut self, memory_page: Page) {
//     self.header = memory_page.header;
//     self.data = memory_page.data; // Here persistence code should happen - Like get relation first, then update in that relation.
// }

// pub fn write_to_disk(&mut self, file: &mut std::fs::File) -> std::io::Result<()> {
//     file.seek(SeekFrom::Start((self.id as u64) * PAGE_SIZE as u64))?;
//     file.write_all(&self.data)?;
//     Ok(())
// }