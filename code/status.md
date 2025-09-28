# Implementation

1. Created Page Data Structure similar to Postgres Page Layout.
2. Page Header contains only lower and upper - for simple storage manager this is enough.
3. ItemId contains only offset - this works for **Fixed size rows**.
    - If length is not required in ItemId then no need of ItemId's also because we can directly calculate data offset.


## On System Start
* Load Catalog File - Now contains only Table Name and Table Location - stored in the form of pages.


## Questions.
1. Page Header Id and Page Number same?
2. Difference betweeen Page::Page() and void init(page_no), we can only one function?
3. load from should take page id of disk, page id of memory, no need of raw. to which page it should load?
4. Page type is missing, like catalog page,relation page etc., Differentiation between in memory pages and disk pages.
    - For example: loads_from - disk page -> memory page
    - stores_to - memory page -> disk page.
5. store_to function should have write_to disk function: Inputs: File Name, page and Outputs: Result
