use playback_rs::{Player, Song};

let player = Player::new(None).expect("Failed to open an audio output.");

#[tauri::command]
fn play_song(song_path: String) {
    let song = Song::from_file(song_path, None).expect("Failed to load or decode the song."); // Decode a song from a file
    player.play(song);
}

#[tauri::command]
fn pause_song() {
    player.set_playing(false);
}

#[tauri::command]
fn resume_song() {
    player.set_playing(true);
}

