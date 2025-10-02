/*
This page is for reference only.
All the page related operations are in raw_page.rs file
*/

pub const PAGE_SIZE: usize = 8192; // Page size
pub const PAGE_HEADER_SIZE: u32 = 8; // Page Header Size - 8 bytes

pub struct Page {
    pub header: PageHeader,
    pub item_id_data: Vec<ItemId>,
}

impl PageHeader {
    pub fn new() -> Self {
        Self {
            lower: 8,
            upper: PAGE_HEADER_SIZE,
        }
    }
}
