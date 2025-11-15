
use serde_json::json;
// use lofty;
// use lofty::file::{AudioFile, TaggedFileExt};
// use tauri::{Emitter, AppHandle};
use std::sync::mpsc;
use std::thread;
use rand::RandomInt;
use meta::{SongMetadata};
use utils::{is_roman_alphabet};

use std::path::PathBuf;
use tauri_plugin_android_fs::{AndroidFsExt, Entry, PublicAudioDir, Result};
use tauri_plugin_android_fs::api::api_sync::AndroidFs;
use tauri::async_runtime::Mutex;
use tauri::EventLoopMessage;

pub fn collect_music_files_android(app: &tauri::AppHandle<impl tauri::Runtime>) -> Result<Vec<PathBuf>> {
    tauri::async_runtime::block_on(async {
        let api = app.android_fs();
        let supported = ["mp3", "m4a", "wav", "flac"];

        if !api.public_storage().request_permission()? {
            return Err(tauri_plugin_android_fs::Error::with("Permission denied by user"));
        }

        let music_dir = api
            .public_storage()
            .resolve_initial_location(
                None,
                PublicAudioDir::Music,
                "",
                false,
            )?;

        fn collect_recursively<R>(
            api: &AndroidFs<R>,
            dir_uri: &tauri_plugin_android_fs::FileUri,
            supported: &[&str],
            acc: &mut Vec<PathBuf>,
        ) -> Result<()> where R: tauri_runtime::Runtime<EventLoopMessage> {
            for entry in api.read_dir(dir_uri)? {
                match entry {
                    Entry::File { uri, name, .. } => {
                        if let Some(ext) = name.split('.').last() {
                            if supported.contains(&ext.to_lowercase().as_str()) {
                                let file_path: tauri_plugin_fs::FilePath = uri.clone().into();
                                if let tauri_plugin_fs::FilePath::Path(p) = file_path {
                                    acc.push(p);
                                }
                            }
                        }
                    }
                    Entry::Dir { uri, .. } => {
                        collect_recursively(api, &uri, supported, acc);
                    }
                }
            }
            Ok(())
        }

        let mut music_files = Vec::new();
        collect_recursively(&api, &music_dir, &supported, &mut music_files);
        Ok(music_files)
    })
}



#[tauri::command]
pub fn scan_music_files(app: &tauri::AppHandle<impl tauri::Runtime>) -> serde_json::Value {
    let music_files: Vec<PathBuf> = collect_music_files_android(app).expect("Failure");
    let mut file_string: String;
    let mut artist: String;
    let mut title: String;
    let mut album: String;
    let mut genre: String;
    let mut entries: Vec<serde_json::Value> = Vec::new();
    for music_file in music_files {
        
        match music_file.to_str() {
            Some(s) => {
                file_string = s.to_string()
            },
            None => file_string = String::from("File String"),
        }
        let metadata = SongMetadata::from_file(music_file).unwrap();
        let artist = match metadata.artist {
            Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
            _ => "Unknown Artist".to_string(),
        };

        let album = match metadata.album {
            Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
            _ => "Unknown Album".to_string(),
        };

        let title = match metadata.title {
            Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
            _ => "Unknown Title".to_string(),
        };

        let genre = match metadata.genre {
            Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone().to_string(),
            _ => "Unknown Genre".to_string(),
        };
        let possible_covers = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17];
        let random_index = RandomInt::new(0, (possible_covers.len() - 1).try_into().unwrap()) as usize;
        let entry = serde_json::json!({
            "location": file_string,
            "artist": artist,
            "album": album,
            "title": title,
            "genre": genre,
            "cover":  format!("covers/{}.avif", possible_covers[random_index]),
        });
        

        entries.push(entry);


    }
    serde_json::Value::Array(entries)
}




#[tauri::command]
pub fn return_genres(music: serde_json::Value) {
    let mut genres = Vec::new();
    for item in music.as_array().unwrap() {
        let genre = item["genre"].as_str().unwrap();
        if !genres.contains(&genre.to_string()) {
            genres.push(genre.to_string());
        }
    }
}

