mod scan;
mod player;
mod logger;
mod filemanager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// This prevents crashes on startup for Android devices.
#[cfg(target_os = "android")]
#[link(name = "c++_shared")]
extern "C" {}

// This is untested.
#[cfg(target_os = "ios")]
#[link(name = "c++")]
extern "C" {}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            player::play_song,
            player::toggle_playing,
            player::stop,
            player::is_song_playing,
            player::get_song_duration,
            player::get_song_progress,
            player::seek_to,
            player::emit_song_progress,
            scan::scan_music_files,
            scan::return_genres,
            logger::log,
            filemanager::get_scan_file,
            filemanager::save_music_data,
            filemanager::confirm_path

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
