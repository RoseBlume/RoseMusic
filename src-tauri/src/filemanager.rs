use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde_json;

use crate::scan;


// filepath: /c:/Users/James/Documents/Code/CipherDen/RoseMusic/src-tauri/src/manage.rs


/// Returns the full path to the scan.json file. If the file does not exist,
/// it will be created using data from scan.rs.
#[tauri::command]
pub async fn get_scan_file() -> serde_json::Value {
    #[cfg(target_os = "android")]
    {
        return scan::scan_music_files().await;
    }

    #[cfg(not(target_os = "android"))]
    {
        let file_path = get_scan_file_path();

        if !file_path.exists() {
            // Run the scan to get music data.
            let scan_data = scan::scan_music_files().await;

            // Ensure the parent directory exists.
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create directory");
            }

            // Write the JSON data to the file.
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
}



/// Returns the local file system path for scan.json for each operating system.
fn get_scan_file_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        // On Windows, use APPDATA\RosaryMusic\scan.json
        let appdata = env::var("APPDATA").expect("APPDATA is not set");
        return PathBuf::from(appdata).join("RosaryMusic").join("scan.json");
    }
    #[cfg(target_os = "macos")]
    {
        // On macOS, use ~/Library/Application Support/RosaryMusic/scan.json
        let home = env::var("HOME").expect("HOME is not set");
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("RosaryMusic")
            .join("scan.json");
    }
    #[cfg(target_os = "linux")]
    {
        // On Linux, use ~/.config/RosaryMusic/scan.json
        let home = env::var("HOME").expect("HOME is not set");
        return PathBuf::from(home).join(".config").join("RosaryMusic").join("scan.json");
    }
    #[cfg(target_os = "android")]
    {
        // On Android, use the app's private directory
        let private_dir = env::var("EXTERNAL_STORAGE").expect("EXTERNAL_STORAGE is not set");
        return PathBuf::from(private_dir)
            .join("Android")
            .join("data")
            .join("com.rosarymusic")
            .join("files")
            .join("scan.json");
   
    }
    // Fallback for other OSes.
    #[allow(unreachable_code)]
    {
        PathBuf::new()
    }
}

/// Returns the username from the environment, depending on the operating system.
pub fn get_username() -> String {
    #[cfg(target_os = "windows")]
    {
        env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }
}

#[tauri::command]
pub fn save_music_data(data: serde_json::Value) {
    let file_path = get_scan_file_path();

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

