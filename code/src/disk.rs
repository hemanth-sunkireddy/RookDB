use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read, ErrorKind, Error, Write};

use crate::page::{Page, PAGE_SIZE, init_page};

// Create Page 
pub fn create_page(file: &mut File) -> io::Result<u32> {
    // Create an empty page (all zeros)
    let mut page = Page::new();

    init_page(&mut page);

    // Get current file size
    let file_size = file.metadata()?.len();

    // Calculate the page number for the new page
    let page_num = (file_size / PAGE_SIZE as u64) as u32;

    // Move cursor to end of file
    file.seek(SeekFrom::End(0))?;

    // Write the zero-filled page
    file.write_all(&page.data)?;

    println!("Created new Page with Id: {} at Offset: {} position in the file.", page_num, file_size);

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