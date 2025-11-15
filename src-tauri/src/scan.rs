

use std::thread;

use utils::get_scan_file;
use tauri::{AppHandle, Emitter};
use serde_json::Value;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FinishedSearching {
  obj: Value,
}

#[tauri::command]
pub fn scan_music_files(app: AppHandle) {
    println!("Updating Data");
    thread::spawn(move || {
        let obj = get_scan_file();
        println!("Finished Searching");
        app.emit("finished-searching", FinishedSearching { obj }).unwrap();
    });
    println!("Finished main function");
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

