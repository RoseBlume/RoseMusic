

// use std::thread;

use utils::{collect_music_files, is_roman_alphabet};
#[cfg(not(debug_assertions))]
use utils::SCANFILE_PATH;
use remeta::SongMetadata;
use rand::RandomInt;
use tauri::{AppHandle, Emitter};
use serde_json::Value;
use serde::Serialize;
// use std::path::PathBuf;
#[cfg(not(debug_assertions))]
use std::fs::{File, self};
#[cfg(not(debug_assertions))]
use std::io::Write;


use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
    sync::{Arc, Mutex},
};
// use tauri::AppHandle;
use serde_json::json;

// const CHUNK_SIZE: usize = 300;

fn calculate_chunk_size(file_count: usize, threads: usize) -> usize {
    file_count / threads.max(1)
}

fn scan_music(app: &AppHandle) -> serde_json::Value {
    let music_files: Vec<PathBuf> = collect_music_files();
    let total_files = music_files.len();
    let total_files_f32 = total_files as f32;

    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let chunk_size = calculate_chunk_size(total_files, threads);

    let chunks: Vec<Vec<PathBuf>> = music_files
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();

    // Shared vector for final output
    let entries = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));

    // Shared counter for progress updates
    let progress_count = Arc::new(Mutex::new(0usize));

    let mut handles = Vec::new();
    let start = Instant::now();
    for chunk in chunks {
        let app_handle = app.clone();
        let entries_ref = Arc::clone(&entries);
        let progress_ref = Arc::clone(&progress_count);

        let handle = thread::spawn(move || {
            for music_file in chunk {
                // --- Progress update ---
                {
                    let mut counter = progress_ref.lock().unwrap();
                    *counter += 1;
                    let pct = ((*counter as f32 / total_files_f32) * 100.0).round() as u32;
                    app_handle.emit("scan-progress", json!({ "percentage": pct })).ok();
                }

                // --- Metadata processing ---
                let file_string = music_file.to_str().unwrap_or("File String").to_string();
                let metadata = SongMetadata::from_file(music_file.clone()).unwrap();

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

                let duration = metadata.duration_ms.unwrap();

                let covers = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17];
                let idx = RandomInt::new(0, ((covers.len() - 1) as i32).try_into().unwrap()) as usize;

                let entry = json!({
                    "location": file_string,
                    "artist": artist,
                    "album": album,
                    "title": title,
                    "genre": genre.trim_start_matches(|c: char| !c.is_ascii_alphabetic()),
                    "duration": duration,
                    "cover": format!("covers/{}.avif", covers[idx]),
                });

                entries_ref.lock().unwrap().push(entry);

                // --- CPU throttling (keeps CPU from hitting 100%) ---
                thread::sleep(Duration::from_millis(1));
            }
        });

        handles.push(handle);
    }

    // Wait for all threads
    for h in handles {
        let _ = h.join();
    }
    let end = start.elapsed();
    println!("{:?}", end);
    // Emit final 100%
    app.emit("scan-progress", json!({ "percentage": 100 })).ok();

    // Return JSON array
    let final_entries = Arc::try_unwrap(entries)
        .unwrap()
        .into_inner()
        .unwrap();

    serde_json::Value::Array(final_entries)
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FinishedSearching {
  obj: Value,
}

#[tauri::command]
pub fn scan_music_files(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("Updating Data");
    thread::spawn(move || {
        #[cfg(debug_assertions)]
        let obj = scan_music(&app);
        #[cfg(not(debug_assertions))]
        let obj = get_scan_file(&app);
        #[cfg(debug_assertions)]
        println!("Finished Searching");
        app.emit("finished-searching", FinishedSearching { obj }).unwrap();
    });
    #[cfg(debug_assertions)]
    println!("Finished main function");
}
#[cfg(not(debug_assertions))]
fn get_scan_file(app: &AppHandle) -> serde_json::Value {
    let file_path = PathBuf::from(&*SCANFILE_PATH);

    if !file_path.exists() {
        let scan_data = scan_music(app);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        let data_string = serde_json::to_string_pretty(&scan_data)
            .expect("Failed to serialize scan data");
        let mut file = File::create(&file_path)
            .expect("Failed to create scan file");
        file.write_all(data_string.as_bytes())
            .expect("Failed to write scan data");
    }

    let data_string = fs::read_to_string(&file_path)
        .expect("Failed to read scan file");
    serde_json::from_str(&data_string)
        .expect("Failed to parse scan file JSON")
}
#[cfg(not(debug_assertions))]
pub fn scan_file_exists() -> bool {
    let file_path = PathBuf::from(&*SCANFILE_PATH);
    if !file_path.exists() {
        false
    }
    else {
        true
        // false
    }
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

