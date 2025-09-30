// pub const PAGE_SIZE: usize = 8192;  // Page size
// pub const PAGE_HEADER_SIZE: u32 = 8; // Page Header Size - 8 bytes

// pub struct PageHeader {
//     pub lower: u32,   // Offset to start of free space - 4 bytes
//     pub upper: u32,   // Offset to end of free space - 4 bytes
// }

// pub struct ItemId {
//     pub offset: u32, // Offset of the item
// }

// pub struct Page {
//     pub header: PageHeader,
//     pub item_id_data: Vec<ItemId>,
//     pub data: Vec<u8>    // Actual Data
// }

// impl PageHeader {
//     pub fn new() -> Self {
//         Self {
//             lower: 8,
//             upper: PAGE_HEADER_SIZE,
//         }
//     }
// }

// impl Page {
//     pub fn new() -> Self {
//         Self { header: PageHeader::new( ), 
//             item_id_data: Vec::new(),
//             data: vec![0; PAGE_SIZE]  // Nee to modify the page size.
//         }
//     }
// }