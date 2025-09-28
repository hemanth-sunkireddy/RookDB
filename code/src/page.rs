pub const PAGE_SIZE: usize = 8192;  // Page size
pub const PAGE_HEADER_SIZE: u16 = 4; // Page Header Size - 4 bytes

pub struct PageHeader {
    pub lower: u16,   // Offset to start of free space - 2 bytes
    pub upper: u16,   // Offset to end of free space - 2 bytes
}

pub struct ItemId {
    pub offset: u16, // Offset of the item
}

pub struct Page {
    pub header: PageHeader,
    pub item_id_data: ItemId,
    pub data: Vec<u8>    // Actual Data
}

impl PageHeader {
    pub fn new() -> Self {
        Self {
            lower: 0,
            upper: PAGE_HEADER_SIZE,
        }
    }
}

impl Page {
    pub fn new() -> Self {
        Self { header: PageHeader::new( ), 
            item_id_data: ItemId { offset: 0 },
            data: Vec::with_capacity(PAGE_SIZE)
        }
    }
}