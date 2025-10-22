use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

pub const DATA_DIR: &str = "database";
pub const CATALOG_DIR: &str = "database/global";
pub const CATALOG_FILE: &str = "database/global/catalog.json";
pub const TABLE_DIR: &str = "database/base";
pub const TABLE_FILE_TEMPLATE: &str = "database/base/{table}.dat";



#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<Column>,
}

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    pub tables: HashMap<String, Table>,
}

pub fn init_catalog() {
    let catalog_path = Path::new(CATALOG_FILE);

    // Step 1: Create directories if not exist
    if let Some(parent) = catalog_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create catalog directory");
        }
    }

    // Step 2: Create catalog.json if not exist
    if !catalog_path.exists() {
        let empty_catalog = Catalog { tables: HashMap::new() };
        let json = serde_json::to_string_pretty(&empty_catalog).expect("Failed to serialize empty catalog");
        fs::write(catalog_path, json).expect("Failed to write catalog file");
        println!("Catalog File Not exist. Created new catalog file at {}", catalog_path.display());
    } else {
        println!("Catalog file already exists at {}", catalog_path.display());
    }
}

pub fn load_catalog() -> Catalog {
    let catalog_path = Path::new(CATALOG_FILE);

    // Check if catalog file exists
    if !catalog_path.exists() {
        eprintln!("Catalog file does not exist at {}.", catalog_path.display());
        return Catalog { tables: HashMap::new() };
    }

    // Read the catalog file
    let data = fs::read_to_string(catalog_path);
    let data = match data {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read catalog file: {}", err);
            return Catalog { tables: HashMap::new() };
        }
    };

    // Deserialize JSON into Catalog struct
    match serde_json::from_str::<Catalog>(&data) {
        Ok(catalog) => catalog,
        Err(err) => {
            eprintln!("Failed to parse catalog JSON: {}", err);
            Catalog { tables: HashMap::new() }
        }
    }
}

