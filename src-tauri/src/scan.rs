use std::path::PathBuf;
use std::env;
use serde_json::json;
use walkdir::WalkDir;
use lofty;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::ItemKey;
use tauri::{Emitter, AppHandle};
use tokio;
use rand::Rng; // Make sure to include the `rand` crate in your Cargo.toml
#[tauri::command]
pub async fn scan_music_files() -> serde_json::Value {

    let music_folder = get_music_folder();
    let supported = ["mp3", "m4a", "wav", "flac", "ogg"];

    let results = tokio::task::spawn_blocking(move || {
        let mut results = Vec::new();
        let possible_covers = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17];
        let mut rng = rand::thread_rng();

        for entry in walkdir::WalkDir::new(music_folder)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())
            {
                if supported.contains(&ext.as_str()) {
                    let (title, artist, genre, duration) = match lofty::read_from_path(path) {
                        Ok(tagged_file) => {
                            let primary = tagged_file.primary_tag();
                            let title = primary
                                .and_then(|t| t.get_string(&lofty::prelude::ItemKey::TrackTitle))
                                .map(String::from)
                                .or_else(|| {
                                    path.file_stem()
                                        .and_then(|s| s.to_str())
                                        .map(String::from)
                                })
                                .unwrap_or_else(|| "Unknown Title".to_string());

                            let artist = primary
                                .and_then(|t| t.get_string(&lofty::prelude::ItemKey::TrackArtist))
                                .map(String::from)
                                .unwrap_or_else(|| "Unknown Artist".to_string());

                            let genre = primary
                                .and_then(|t| t.get_string(&lofty::prelude::ItemKey::Genre))
                                .map(String::from)
                                .unwrap_or_else(|| "Unknown Genre".to_string());

                            let duration = (tagged_file.properties().duration().as_millis()) as u64;

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

                    // Add cover image selection
                    let random_index = rng.gen_range(0..possible_covers.len());
                    let cover = format!("covers/{}.avif", possible_covers[random_index]);

                    let item = serde_json::json!({
                        "title": title,
                        "artist": artist,
                        "genre": genre,
                        "duration": duration,
                        "location": path.to_string_lossy(),
                        "cover": cover
                    });
                    results.push(item);
                }
            }
        }

        results
    })
    .await
    .unwrap();

    serde_json::Value::Array(results)
}

fn get_music_folder() -> PathBuf {
    // Windows: Use USERPROFILE and "Music"
    #[cfg(target_os = "windows")]
    {
        let home = env::var("USERPROFILE").expect("USERPROFILE is not set");
        return PathBuf::from(home).join("Music");
    }

    // macOS and iOS: Use HOME and "Music"
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        let home = env::var("HOME").expect("HOME is not set");
        return PathBuf::from(home).join("Music");
    }

    // Android: Typically HOME might be set, or adjust if needed
    #[cfg(target_os = "android")]
    {
        return PathBuf::from("/storage/emulated/0/Music");
    }

    // Linux: Use HOME and "Music"
    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").expect("HOME is not set");
        return PathBuf::from(home).join("Music");
    }

    // Fallback
    #[allow(unreachable_code)]
    {
        PathBuf::new()
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