use std::fs::File;
use std::io;

pub const PAGE_SIZE: usize = 8192; // Page size - storing as 8 bytes and as usize only because most pointers(file pointers and otheres requires it to be 8 bytes)
// pub const PAGE_HEADER_SIZE: u32 = 8;
pub const ITEM_ID_SIZE: u32  = 8;

use crate::disk::{read_page, write_page};

// pub struct PageHeader {
//     pub lower: u32, // Offset to start of free space - 4 bytes
//     pub upper: u32, // Offset to end of free space - 4 bytes
// }

// pub struct ItemId {
//     pub offset: u32, // Offset of the item or Tuple - 4 bytes
//     pub length: u32, // Length of the item or Tuple - 4 bytes
// }

pub struct Page {
    pub data: Vec<u8>, // Fixed-size buffer holds the raw bytes of a page (PAGE_SIZE = 8KB)
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: vec![0; PAGE_SIZE],
        }
    }
}

pub fn page_count(file: &mut File) -> io::Result<u32> {
    // Get current file size
    let file_size = file.metadata()?.len(); // u64
    let total_pages = file_size / PAGE_SIZE as u64;
    Ok(total_pages as u32)
}

pub fn page_free_space(page: &Page) -> io::Result<u32> {
    // Get Lower and Upper of page
    let lower = u32::from_be_bytes(page.data[0..4].try_into().unwrap());
    let upper = u32::from_be_bytes(page.data[4..8].try_into().unwrap());

    // Freespace is equal to upper - lower
    Ok(upper - lower)
}

pub fn page_add_data(file: &mut File, data: &[u8]) -> io::Result<()> {
    // First get total number of Pages
    let total_pages: u32 = page_count(file)?;

    // Go to the last page
    let last_page_num: u32 = total_pages - 1;

    let mut last_page: Page = Page::new();
    read_page(file, &mut last_page, last_page_num)?;

    // Check the free space in last page
    let free_space: u32 = page_free_space(&last_page)?;

    println!("Free Space in Last Page: { }", free_space);

    // Check if data fits in last page
    // Total Space required = Size(data) + Item Id Size
    if data.len() as u32 + ITEM_ID_SIZE <= free_space {
        // Last page have space - Add it here
        let upper = u32::from_be_bytes(last_page.data[4..8].try_into().unwrap());
        let start = upper - data.len() as u32;

        last_page.data[start as usize..upper as usize].copy_from_slice(data);

        // Update upper pointer
        last_page.data[4..8].copy_from_slice(&start.to_be_bytes());

        // TODO: update Item Id Data also at the lower point.

        // Write back the page
        write_page(file, &mut last_page, last_page_num)?;
        println!("Data added to page {}", last_page_num);
    } else {
        // Not enough space → TODO: create a new page and insert there
        println!("Not enough space in last page. Need a new page.");
    }

    Ok(())
}
