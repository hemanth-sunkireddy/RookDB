// use std::io;
use std::fs;
use std::io::{ Write, Seek};

pub mod page;
// use crate::page::Page;

pub mod catalog;
use crate::catalog::{Catalog};

pub const CATALOG_PATH: &str = "database/global/catalog.dat"; // Catalog file path

fn main() {
    println!("Welcome to Storage Manager");

    // Serialize total_pages = 1 into 4 bytes (little endian)
    let total_pages_bytes = 1u32.to_be_bytes();

    // Overwrite first 4 bytes of catalog.dat
    let mut catalog_file = fs::OpenOptions::new()
        .write(true)
        .open(CATALOG_PATH)
        .expect("Failed to open catalog file");

    // Write the 4 bytes at the beginning
    catalog_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    catalog_file.write_all(&total_pages_bytes).unwrap();

    println!("Catalog header updated: total_pages = 1");

    // Create Page in Memory
    // let page = Page::new();
    // println!("Created 1 Page in Memory.");

    
    // Loading Catalog
    let catalog_bytes = fs::read(CATALOG_PATH).unwrap();
    println!("Raw Data Inside Catalog: {:?}", catalog_bytes);

    // Loading Catalog
    let catalog_bytes = fs::read(CATALOG_PATH).unwrap();
    println!("Data Inside Catalog: {:?}", catalog_bytes);

    // Parse catalog from bytes
    let catalog = Catalog::parse(&catalog_bytes);
    println!("Catalog Header -> total_pages = {}", catalog.header.total_pages);
}
