#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(not(target_os = "android"))]
pub use desktop::{scan_music_files, return_genres};

#[cfg(target_os = "android")]
mod mobile;
#[cfg(target_os = "android")]
pub use mobile::{scan_music_files, return_genres};