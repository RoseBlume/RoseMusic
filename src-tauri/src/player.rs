use rodio::{
    Decoder,
    stream::OutputStream, 
    OutputStreamBuilder, 
    Sink, 
    Source
};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Mutex, Arc};
use std::error::Error;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;

struct Player {
    // Keep the stream alive for sink playback
    _stream: OutputStream,
    stream_handle: OutputStream,
    sink: Sink,
    start_time: Instant,
    paused_time: Option<Instant>,
    elapsed_paused: Duration,
    duration: Option<Duration>,
    file_location: String,
}

impl Player {
    fn new(location: &str) -> Result<Self, Box<dyn Error>> {
        
        let _stream = OutputStreamBuilder::open_default_stream()?;
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());
        let file = File::open(location)?;
        let source = Decoder::new(BufReader::new(file))?;
        let duration = source.total_duration();
        let agc_source = source.automatic_gain_control(1.0, 4.0, 0.0, 5.0);
        let agc_enabled = Arc::new(AtomicBool::new(true));
        let agc_enabled_clone = agc_enabled.clone();
        let controlled = agc_source.periodic_access(Duration::from_millis(5), move |agc_source| {
            agc_source.set_enabled(agc_enabled_clone.load(Ordering::Relaxed));
        });
        
        let start_time = Instant::now();
        
        // sink.append(source);
        sink.append(controlled);

        Ok(Player {
            _stream,
            stream_handle,
            sink,
            start_time,
            paused_time: None,
            elapsed_paused: Duration::default(),
            duration,
            file_location: location.to_string(),
        })
    }

    fn toggle_playing(&mut self) {
        if self.sink.is_paused() {
            self.sink.play();
            if let Some(paused_time) = self.paused_time {
                self.elapsed_paused += paused_time.elapsed();
                self.paused_time = None;
            }
        } else {
            self.sink.pause();
            self.paused_time = Some(Instant::now());
        }
    }

    fn stop(&self) {
        self.sink.stop();
    }
    
    fn song_duration_ms(&self) -> u64 {
        // Returns 0 if duration is unknown.
        self.duration.unwrap_or_default().as_millis() as u64
    }

    fn song_progress_ms(&self) -> u64 {
        // Compute progress from when the song was started, accounting for pauses.
        let elapsed = if let Some(paused_time) = self.paused_time {
            self.start_time.elapsed() - self.elapsed_paused - paused_time.elapsed()
        } else {
            self.start_time.elapsed() - self.elapsed_paused
        };
        elapsed.as_millis() as u64
    }

    
}

static PLAYER: Lazy<Mutex<Option<Player>>> = Lazy::new(|| Mutex::new(None));

/// Plays a song from the given file location.
/// This function creates a new player instance and starts playing the song.
#[tauri::command]
pub fn play_song(location: &str) {
    begin_song(location).expect("Failed to play song");
}

/// Toggles between pausing and resuming the current song.
#[tauri::command]
pub fn toggle_playing() {
    if let Some(ref mut player) = *PLAYER.lock().unwrap() {
        player.toggle_playing();
    }
}

/// Stops the current song playback.
#[tauri::command]
pub fn stop() {
    end_song().unwrap();
}

/// Checks if the song is still playing.
/// Returns true if audio is not finished playing.
#[tauri::command]
pub fn is_song_playing() -> bool {
    if let Some(ref player) = *PLAYER.lock().unwrap() {
        !player.sink.empty()
    } else {
        false
    }
}


fn begin_song(location: &str) -> Result<(), Box<dyn std::error::Error>> {
    let player = Player::new(location)?;
    let mut current = PLAYER.lock().unwrap();
    *current = Some(player);
    Ok(())
}

fn end_song() -> Result<(), Box<dyn std::error::Error>> {
    let mut current = PLAYER.lock().unwrap();
    if let Some(player) = current.take() {
        player.stop();
    }
    Ok(())
}

#[tauri::command]
pub fn get_song_duration() -> u64 {
    PLAYER
        .lock()
        .unwrap()
        .as_ref()
        .map(|p| p.song_duration_ms())
        .unwrap_or(0)
}

#[tauri::command]
pub fn get_song_progress() -> u64 {
    PLAYER
        .lock()
        .unwrap()
        .as_ref()
        .map(|p| p.song_progress_ms())
        .unwrap_or(0)
}

#[tauri::command]
pub fn seek_to(time_ms: u64) {
    seek(time_ms).unwrap();
}

pub fn seek(time_ms: u64) -> Result<(), Box<dyn Error>> {
    use rodio::Source;
    let mut player_guard = PLAYER.lock().unwrap();
    if let Some(ref mut player) = *player_guard {
        let skip = Duration::from_millis(time_ms);
        // Stop the current sink and recreate it with a source that skips the specified duration.
        player.sink.stop();
        let file = File::open(&player.file_location)?;
        let source = Decoder::new(BufReader::new(file))?.skip_duration(skip);
        let new_sink = Sink::connect_new(&player.stream_handle.mixer());
        new_sink.append(source);
        player.sink = new_sink;
        player.start_time = Instant::now() - skip;
        player.elapsed_paused = Duration::default();
        player.paused_time = None;
    }
    Ok(())
}


unsafe impl Send for Player {}