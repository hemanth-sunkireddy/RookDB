## Storage Manager Design Doc - version 0

## Page
```rust
pub struct Page {
    pub data: Vec<u8>  // Fixed-size buffer holds the raw bytes of a page (PAGE_SIZE = 8KB)
}
```

---

### 0.`create_page` API
**Description:**  
Create a page in disk for a file.

**Function:**  
```rust
pub fn create_page(file: &mut File)
```
**Input:** 
`file:` file to create to a file

**Output:** 
Create a page at the end of the file.

**Implementation:**
1. Initializes a new page in memory with all zeros (PAGE_SIZE bytes).
2. Moves the file cursor to the end of the file.
3. Writes the entire zero-filled page to the file, effectively creating a new page on disk.
---

### 1. `read_page` API
**Description:**  
Reads a page from a disk/file into memory.

**Function:**  
```rust
pub fn read_page(file: &mut File, page: &mut Page, page_num: u32)
```
**Input:** 
`file:` file to read from, 
`page:` memory page to fill, 
`page_num:` page number to read

**Output:** 
Populates the given memory page with data read from the file.

**Implementation:**
1. Calculates the **offset** as `page_num * PAGE_SIZE` and moves the file cursor to the correct position.
2. Reads data from that offset position up to `offset + PAGE_SIZE` and copies it into the page memory.

**Cases Handled:**
1. Checks the file size and returns an error if the requested page does not exist in the file.

---

### 2.`write_page` API
**Description:**  
Write a page from memory to disk/file.

**Function:**  
```rust
pub fn write_page(file: &mut File, page: &mut Page, page_num: u32)
```
**Input:** 
`file:` file to write, 
`page:` memory page to copy from, 
`page_num:` page number to write

**Output:** 
Writes the contents of the given memory page to the file at the specified page offset.

**Implementation:**
1. Calculates the **offset** as `page_num * PAGE_SIZE` and moves the file cursor to the correct position.
2. copy the contents of the given memory page from offset to `offset + PAGE_SIZE` positions to the file.

---



<!-- Few Design choices to consider:
1. In Read and Write page should we pass file name or File pointer.
2. Because in create page passing file name is better than passing file pointer. -->