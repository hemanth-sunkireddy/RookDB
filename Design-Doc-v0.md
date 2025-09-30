## Storage Manager Design Doc - version 0

## Page
```rust
pub struct Page {
    pub data: Vec<u8>
}
```

### `read_page` API
**Description:**  
Reads a page from a disk file into memory.

**Function:**  
```rust
pub fn read_page(file: &mut File, page: &mut Page, page_num: u32)
```
**Input:** 
`file:` – file to read from, 
`page:` – memory page to fill, 
`page_num:` – page number to read

**Output:** 
Populates the given memory page with data read from the file.

**Implementation:**
1. Calculates the **offset** as `page_num * PAGE_SIZE` and moves the file cursor to the correct position.
2. Reads data from that offset position up to `offset + PAGE_SIZE` and copies it into the page memory.

**Cases Handled:**
1. Checks the file size and returns an error if the requested page does not exist in the file.
