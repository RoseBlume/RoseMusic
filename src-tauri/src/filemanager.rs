use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde_json;
use crate::scan;
// use crate::scan::{self, USERNAME};
use utils::{SCANFILE_PATH};
// filepath: /c:/Users/James/Documents/Code/CipherDen/RoseMusic/src-tauri/src/manage.rs


/// Returns the full path to the scan.json file. If the file does not exist,
/// it will be created using data from scan.rs.
#[tauri::command]
pub fn get_scan_file() -> serde_json::Value {
 
    let file_path = PathBuf::from(&*SCANFILE_PATH);

    if !file_path.exists() {
        // Run the scan to get music data.
        // let scan_data = scan::scan_music_files();

        // // Ensure the parent directory exists.
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        // // Write the JSON data to the file.
        let data_string = serde_json::to_string_pretty(&scan_data)
            .expect("Failed to serialize scan data");
        let mut file = File::create(&file_path)
            .expect("Failed to create scan file");
        file.write_all(data_string.as_bytes())
            .expect("Failed to write scan data");
    }

    // Read and parse the JSON data from file.
    let data_string = fs::read_to_string(&file_path)
        .expect("Failed to read scan file");
    serde_json::from_str(&data_string)
        .expect("Failed to parse scan file JSON")
    
}






#[tauri::command]
pub fn save_music_data(data: serde_json::Value) {
    let file_path = PathBuf::from(&*SCANFILE_PATH);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let data_string = serde_json::to_string_pretty(&data).expect("Failed to serialize music data");

    let mut file = File::create(&file_path).expect("Failed to create music data file");
    file.write_all(data_string.as_bytes()).expect("Failed to write music data");
}


#[tauri::command]
pub fn confirm_path(path: &str) -> bool {
    PathBuf::from(path).exists()
}

