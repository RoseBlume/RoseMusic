// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
/*
use std::fs;
use std::path::PathBuf;
use whoami;
use serde::Serialize;
use serde_json::json;

// mod player;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_music_folder() -> String {
    #[cfg(target_os = "windows")]
    {
        let username = whoami::username();
        return format!("C:/Users/{}/Music", username);
    }

    #[cfg(target_os = "linux")]
    {
        let username = whoami::username();
        return dirs::audio_dir().unwrap_or_else(|| PathBuf::from(format!("/home/{}/Music", username))).to_str().unwrap().to_string();
    }

    #[cfg(target_os = "android")]
    {
        return "/storage/emulated/0/Music".to_string();
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
    {
        panic!("Unsupported OS");
    }
}

#[derive(Serialize)]
struct MusicFile {
    title: String,
    src: String,

    }
#[tauri::command]
fn scan_music() -> String {
    let music_dir: PathBuf;

    #[cfg(target_os = "windows")]
    {
        let username = whoami::username();
        music_dir = PathBuf::from(format!("C:/Users/{}/Music", username));
    }

    #[cfg(target_os = "linux")]
    {
        let username = whoami::username();
        music_dir =
            dirs::audio_dir().unwrap_or_else(|| PathBuf::from(format!("/home/{}/Music", username)));
    }

    #[cfg(target_os = "android")]
    {
        music_dir = PathBuf::from("/storage/emulated/0/Music");
    }

    let mut music_files = vec![];
    if let Ok(paths) = fs::read_dir(music_dir) {
        for path in paths {
            if let Ok(entry) = path {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    // Check if the file has a music file extension
                    if path_str.ends_with(".mp3") || path_str.ends_with(".wav") || path_str.ends_with(".flac") {
                        // Remove the directory path and file extension, replace underscores with spaces
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let title = file_name.replace(".mp3", "")
                                             .replace(".wav", "")
                                             .replace(".flac", "")
                                             .replace("_", " ");
                        music_files.push(MusicFile {
                            title,
                            src: path_str.to_string(),
                        });
                    }
                }
            }
        }
    }

    serde_json::to_string(&music_files).unwrap()
}
*/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    /*
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_shell::init())
        */
        //.invoke_handler(tauri::generate_handler![greet, scan_music])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
