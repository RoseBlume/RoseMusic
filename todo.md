

1. Fix this for `scan.rs`
```rs
use serde_json::json;
use std::sync::mpsc;
use std::thread;
use rand::RandomInt;
use utils::MUSIC_FOLDER_PATH;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use meta::SongMetadata; // assuming your struct & parser are in this crate

/// Recursively collects all file paths in a directory.
fn collect_files_recursively(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files_recursively(&path)?);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

#[tauri::command]
pub fn scan_music_files() -> serde_json::Value {
    let music_folder_path = PathBuf::from(&*MUSIC_FOLDER_PATH);
    let supported = ["mp3", "m4a", "wav", "flac"];
    let supported_ext: Vec<String> = supported.iter().map(|s| s.to_string()).collect();

    // Collect all music files recursively
    let files = collect_files_recursively(&music_folder_path)
        .unwrap_or_else(|_| Vec::new())
        .into_iter()
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|ext| supported_ext.contains(&ext.to_lowercase()))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    for file in &files {
        println!("This is the path: {}", file.display());
    }

    // Split into manageable chunks
    let block_size = 500;
    let blocks: Vec<Vec<PathBuf>> = files.chunks(block_size).map(|c| c.to_vec()).collect();

    let max_threads = std::cmp::max(num_cpus::get() / 4, 1);
    let (tx, rx) = mpsc::channel();
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();

    for block in blocks {
        let tx = tx.clone();
        let supported_ext = supported_ext.clone();

        while handles.len() >= max_threads {
            if let Some(h) = handles.pop() {
                let _ = h.join();
            }
        }

        let handle = thread::spawn(move || {
            let possible_covers = [
                1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17,
            ];
            let mut block_results = Vec::new();

            for path in block {
                if let Some(ext) = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
                {
                    if supported_ext.contains(&ext) {
                        let metadata = SongMetadata::from_file(&path);
                        let (title, artist, genre) = match metadata {
                            Ok(meta) => (
                                meta.title.unwrap_or_else(|| "Unknown Title".to_string()),
                                meta.artist.unwrap_or_else(|| "Unknown Artist".to_string()),
                                meta.genre.unwrap_or_else(|| "Unknown Genre".to_string()),
                            ),
                            Err(_) => (
                                path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown Title")
                                    .to_string(),
                                "Unknown Artist".to_string(),
                                "Unknown Genre".to_string(),
                            ),
                        };

                        let duration = 0u64; // You can add your duration logic later

                        // Assign random cover
                        let random_index = RandomInt::new(0, (possible_covers.len() - 1) as u32) as usize;
                        let cover = format!("covers/{}.avif", possible_covers[random_index]);

                        let item = json!({
                            "title": title,
                            "artist": artist,
                            "genre": genre,
                            "duration": duration,
                            "location": path.display().to_string(),
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

    // Wait for threads
    for handle in handles {
        let _ = handle.join();
    }

    drop(tx);
    let results: Vec<_> = rx.into_iter().flatten().collect();
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
```



**Note:** This causes an error in which there is an audio stream error which causes the sink to drop. This causes repeated skipping of songs.

2. Implement custom metadata solution

3. Create the radio commands to allow the user to add and remove radio stations.

4. Create the playlist commands to allow the user to create new playlists and add songs to a specific playlist.