use crate::page::{Page};

pub const CATALOG_SIZE: usize = 8192; // Catalog size = 1 page
pub const CATALOG_HEADER_SIZE: u16 = 2; // Catalog Header Size - 2 bytes

pub struct CatalogHeader {
    pub total_pages: u32, // number of pages in catalog - 4 bytes to store this value
}

impl CatalogHeader {
    pub fn new() -> Self {
        Self { total_pages: 1 } // start with 1 page
    }
}

pub struct Catalog {
    pub header: CatalogHeader,
    pub pages: Vec<Page>, // list of pages in catalog
}

impl Catalog {
    pub fn new() -> Self {
        let mut pages = Vec::new();
        pages.push(Page::new()); // create first page

        Self {
            header: CatalogHeader::new(),
            pages,
        }
    }

    /// Parse a Catalog from raw bytes (deserialize header only for now)
    pub fn parse(bytes: &[u8]) -> Self {
        if bytes.len() < 4 {
            panic!("Catalog file too small to contain header!");
        }

        let total_pages = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        Self {
            header: CatalogHeader { total_pages },
            pages: Vec::new(), // later: load real pages from file
        }
    }
}
