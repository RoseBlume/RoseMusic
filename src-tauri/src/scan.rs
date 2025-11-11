use std::path::PathBuf;
use std::env;
use std::result::Result;
use serde_json::json;
use walkdir::WalkDir;
use lofty;
use lofty::file::{AudioFile, TaggedFileExt};
// use tauri::{Emitter, AppHandle};
use std::sync::mpsc;
use std::thread;
use rand::RandomInt;
use utils::{MUSIC_FOLDER_PATH};


#[tauri::command]
pub fn scan_music_files() -> serde_json::Value {
    // let music_folder = *MUSIC_FOLDER_PATH;
    let supported = ["mp3", "m4a", "wav", "flac"];

    // Collect all file paths first
    // println!("Scanning");
    let files: Vec<PathBuf> = WalkDir::new(&*MUSIC_FOLDER_PATH)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Split files into chunks for processing
    let block_size = 500;
    let blocks: Vec<Vec<PathBuf>> = files
        .chunks(block_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Determine how many threads to use
    let max_threads: u8 = (num_cpus::get() / 4).max(1).try_into().unwrap();

    // On Android, force single-thread mode
    #[cfg(target_os = "android")]
    {
        let max_threads = 1;
    }
    let (tx, rx) = mpsc::channel();

    // Thread pool management
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let supported_ext = supported.map(|s| s.to_string());

    for (_i, block) in blocks.into_iter().enumerate() {
        let tx = tx.clone();
        let supported_ext = supported_ext.clone();

        // Limit concurrent threads
        while handles.len() >= max_threads.into() {
            if let Some(h) = handles.pop() {
                let _ = h.join(); // Wait for a thread to finish before spawning another
            }
        }

        let handle = thread::spawn(move || {
            // let mut rng = rand::thread_rng();
            let possible_covers = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17];
            let mut block_results = Vec::new();

            for path in block {
                if let Some(ext) = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
                {
                    if supported_ext.contains(&ext) {
                        let (title, artist, genre, duration) = match lofty::read_from_path(&path) {
                            Ok(tagged_file) => {
                                let primary = tagged_file.primary_tag();
                                let title = primary
                                    .and_then(|t| t.get_string(&lofty::prelude::ItemKey::TrackTitle).map(|s| s.trim()))
                                    .map(String::from)
                                    .or_else(|| {
                                        path.file_stem()
                                            .and_then(|s| s.to_str())
                                            .map(String::from)
                                    })
                                    .unwrap_or_else(|| "Unknown Title".to_string());

                                if title.trim().is_empty() {
                                    continue;
                                }

                                let artist = primary
                                    .and_then(|t| t.get_string(&lofty::prelude::ItemKey::TrackArtist))
                                    .map(String::from)
                                    .unwrap_or_else(|| "Unknown Artist".to_string());

                                let genre = primary
                                    .and_then(|t| t.get_string(&lofty::prelude::ItemKey::Genre))
                                    .map(String::from)
                                    .unwrap_or_else(|| "Unknown Genre".to_string());

                                let duration = tagged_file.properties().duration().as_millis() as u64;

                                (title, artist, genre, duration)
                            }
                            Err(_) => {
                                let title = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .map(String::from)
                                    .unwrap_or_else(|| "Unknown Title".to_string());
                                (title, "Unknown Artist".to_string(), "Unknown Genre".to_string(), 0)
                            }
                        };

                        let random_index = RandomInt::new(0, (possible_covers.len() - 1).try_into().unwrap()) as usize;
                        let cover = format!("covers/{}.avif", possible_covers[random_index]);

                        let item = json!({
                            "title": title,
                            "artist": artist,
                            "genre": genre,
                            "duration": duration,
                            "location": path.to_string_lossy(),
                            "cover": cover
                        });

                        block_results.push(item);
                    }
                }
            }

            tx.send(block_results).ok();
        });

        handles.push(handle);
    }

    // Wait for remaining threads
    for handle in handles {
        let _ = handle.join();
    }

    drop(tx); // Close channel

    // Collect all thread results
    let results: Vec<_> = rx.into_iter().flatten().collect();
    // list_all_music_files();

    serde_json::Value::Array(results)
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


