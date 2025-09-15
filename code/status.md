# Implementation Status

1. Created a page struct.
2. Initliased n pages in memory.
3. Read catalog file using OS file system.


## Questions.
1. Page Header Id and Page Number same?
2. Difference betweeen Page::Page() and void init(page_no), we can only one function?
3. load from should take page id of disk, page id of memory, no need of raw. to which page it should load?
4. Page type is missing, like catalog page,relation page etc., Differentiation between in memory pages and disk pages.
    - For example: loads_from - disk page -> memory page
    - stores_to - memory page -> disk page.
5. store_to function should have write_to disk function: Inputs: File Name, page and Outputs: Result
