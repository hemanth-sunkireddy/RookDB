// use std::io;
use std::fs;

pub mod page;
use crate::page::Page;

pub const MAX_PAGES: usize = 1024;  // 1024 * 8 = 8,388,608 bytes ≈ 8 MB
pub const CATALOG_PATH: &str = "database/catalog.dat"; // Catalog file path

fn main() {
    println!("Welcome to Storage Manager");

    // Taking input from terminal
    // let mut catalog_path = String::new();
    // if let Err(e) = io::stdin().read_line(&mut catalog_path){
    //     eprintln!("Error in Recieving input: {}", e);
    // }
    // print!("Input Recieved: {}", catalog_path);

    // Page Initialisation
    // let page = Page::new(0);
    // println!("Page ID: {}", page.id);
    // println!("First byte of page: {}", page.data[0]);

    // Initialise 1024 pages in Memory
    let mut pages: Vec<Page> = Vec::new();
    for id in 0..MAX_PAGES {
        pages.push(Page::new(id));
        println!("Initliased new page with Id: {}", id);
    }
    println!("Initialised 1024 Pages - Total size: 8 MB in Memory.");

    // Loading Catalog
    let catalog_bytes = fs::read(CATALOG_PATH).unwrap();
    println!("Data Inside Catalog: {:?}", catalog_bytes);

    // Load Page from Disk
    let page = Page::new(32);
    println!("PAGE ID: {}", page.id);

}
