use std::fs::OpenOptions;
use std::io;

mod disk;
mod raw_page;
use crate::disk::{create_page, read_page, write_page};
use crate::raw_page::{Page, page_add_data};

pub const CATALOG_PATH: &str = "database/global/catalog.dat"; // Catalog file path

fn main() -> io::Result<()> {
    println!("Welcome to Storage Manager");

    // Create File Pointer
    let mut file_pointer = OpenOptions::new()
        .read(true)
        .write(true)
        .open(CATALOG_PATH)?;

    /*
    Create a Page in file
    */
    create_page(&mut file_pointer)?;
    println!("Page created successfully.");

    // Create a Page in Memory
    let mut page: Page = Page::new();

    let content = b"Hello, Storage Manager!";
    page.data[..content.len()].copy_from_slice(content);

    let page_num: u32 = 0;

    // Write page to file
    write_page(&mut file_pointer, &mut page, page_num)?;
    println!("Updated Page with content Successfully.");

    /*
    Reading a Page from Disk file to Memory Page
    File: Catalog
    Page: page
    PageNum: 0
    */
    read_page(&mut file_pointer, &mut page, page_num)?;
    let page_text = String::from_utf8_lossy(&page.data);
    println!("Page Data: {}", page_text);

    // Adding Data to the File
    let data_to_add = b"This is some raw data to add to the file.";
    page_add_data(&mut file_pointer, data_to_add)?;
    println!("Data added to file.");

    Ok(())
}

// fn main() {
//     println!("Welcome to Storage Manager");

//     // Serialize total_pages = 1 into 4 bytes (little endian)
//     let total_pages_bytes = 1u32.to_be_bytes();

//     // Overwrite first 4 bytes of catalog.dat
//     let mut catalog_file = fs::OpenOptions::new()
//         .write(true)
//         .open(CATALOG_PATH)
//         .expect("Failed to open catalog file");

//     // Write the 4 bytes at the beginning
//     catalog_file.seek(std::io::SeekFrom::Start(0)).unwrap();
//     catalog_file.write_all(&total_pages_bytes).unwrap();

//     println!("Catalog header updated: total_pages = 1");

//     // Create Page in Memory
//     // let page = Page::new();
//     // println!("Created 1 Page in Memory.");

//     // Loading Catalog
//     let catalog_bytes = fs::read(CATALOG_PATH).unwrap();
//     println!("Raw Data Inside Catalog: {:?}", catalog_bytes);

//     // Loading Catalog
//     let catalog_bytes = fs::read(CATALOG_PATH).unwrap();
//     println!("Data Inside Catalog: {:?}", catalog_bytes);

//     // Parse catalog from bytes
//     let catalog = Catalog::read_catalog(&catalog_bytes);
//     println!("Catalog Header -> total_pages = {}", catalog.header.total_pages);
// }
