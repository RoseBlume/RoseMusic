mod scan;
mod player;
mod logger;

use tauri::{AppHandle, Manager, WindowEvent};
#[cfg(all(
    any(target_os = "windows", target_os = "macos", target_os = "linux"),
    not(debug_assertions)
))]
use tauri::{UserAttentionType::Informational};
use std::thread;
// #[cfg(target_os = "android")]
// mod android;

// #[cfg(target_os = "android")]
// use android::{android_has_all_files_access, android_request_all_files_access};


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
    // #[cfg(target_os = "android")] {
    //     // Now you can call your functions
    //     if !android_has_all_files_access() {
    //         android_request_all_files_access();
    //     }
    // }
    tauri::Builder::default()
        
        .invoke_handler(tauri::generate_handler![
            player::play_song,
            player::toggle_playing,
            player::stop,
            player::is_song_playing,
            player::get_song_duration,
            player::get_song_progress,
            player::seek_to,
            // player::emit_song_progress,
            scan::scan_music_files,
            scan::return_genres,
            set_complete,
            logger::log
            // filemanager::get_scan_file,
            // filemanager::save_music_data,
            // filemanager::confirm_path

        ])
        .setup(|app | {
            // Spawn setup as a non-blocking task so the windows can be
            // created and ran while it executes
            let app_handle = app.handle().clone();
            thread::spawn(move || {
                initialize_windows(&app_handle);
                
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn initialize_windows(app: &AppHandle) {
    let splash_window = app.get_webview_window("splashscreen").unwrap();
    #[cfg(all(
        any(target_os = "windows", target_os = "macos", target_os = "linux"),
        not(debug_assertions)
    ))]
    let main_window = app.get_webview_window("main").unwrap();
    main_window.on_window_event(|event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
          // Allow the window to close
          api.prevent_close(); 

          // Quit the entire app
          std::process::exit(0);
        }
      });
    #[cfg(not(debug_assertions))] {
        if scan::scan_file_exists() {
            main_window.show().unwrap();
        }
        else {
            #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
            splash_window.center().unwrap();
            splash_window.show().unwrap();
            #[cfg(all(
                any(target_os = "windows", target_os = "macos", target_os = "linux"),
                not(debug_assertions)
            ))]
            main_window.request_user_attention(Some(Informational)).unwrap();
        }
    }
    #[cfg(debug_assertions)] {
        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        splash_window.center().unwrap();
        splash_window.show().unwrap();
        #[cfg(all(
            any(target_os = "windows", target_os = "macos", target_os = "linux"),
        ))]
        main_window.request_user_attention(Some(Informational)).unwrap();
    }
}

#[tauri::command]
async fn set_complete(
    app: AppHandle,
) {
    // Lock the state without write access
    // Check if both tasks are completed
    
    let splash_window = app.get_webview_window("splashscreen").unwrap();
    let main_window = app.get_webview_window("main").unwrap();
    splash_window.close().unwrap();
    main_window.show().unwrap();
    #[cfg(all(
        any(target_os = "windows", target_os = "macos", target_os = "linux"),
        not(debug_assertions)
    ))] 
    main_window.request_user_attention(Some(Informational)).unwrap();
}

