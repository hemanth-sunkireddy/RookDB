use std::fs;
use std::path::Path;

use storage_manager::catalog::{
    init_catalog, load_catalog, save_catalog, CATALOG_FILE, Table, Column,
};

#[test]
fn test_save_catalog() {
    // Step 1: Ensure the catalog file exists (create if missing)
    if !Path::new(CATALOG_FILE).exists() {
        init_catalog();
    }

    // Step 2: Load catalog into memory
    let mut catalog = load_catalog();

    // Step 3: Add a new test table entry
    let test_table = Table {
        columns: vec![
            Column {
                name: "id".to_string(),
                data_type: "INT".to_string(),
            },
            Column {
                name: "name".to_string(),
                data_type: "TEXT".to_string(),
            },
            Column {
                name: "email".to_string(),
                data_type: "TEXT".to_string(),
            },
        ],
    };

    catalog.tables.insert("users".to_string(), test_table);

    // Step 4: Save catalog back to disk
    save_catalog(&catalog);

    // Step 5: Reload catalog from disk and verify it contains the new table
    let reloaded_catalog = load_catalog();

    assert!(
        reloaded_catalog.tables.contains_key("users"),
        "Saved catalog does not contain 'users' table"
    );

    let users_table = reloaded_catalog.tables.get("users").unwrap();
    assert_eq!(users_table.columns.len(), 3, "Expected 3 columns in 'users' table");

    // Step 6: Clean up (optional)
    fs::remove_file(CATALOG_FILE).expect("Failed to clean up test catalog.json");
}
