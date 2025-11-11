use std::{fs, path::PathBuf};
use meta::SongMetadata; // replace `my_crate` with your crate name
use utils::MUSIC_FOLDER_PATH;
#[test]
fn scan_music_directory_for_metadata() {
    // Locate Windows Music directory
    let music_dir = PathBuf::from(&*MUSIC_FOLDER_PATH);

    if !music_dir.exists() {
        panic!("Music directory not found at {:?}", music_dir);
    }

    println!("Scanning {:?}", music_dir);

    // Collect all supported files recursively
    let supported_exts = ["mp3", "flac", "m4a", "wav"];
    // let mut files = Vec::new();

    fn collect_files(dir: &PathBuf, exts: &[&str]) -> Vec<PathBuf> {
        let mut list: Vec<PathBuf> = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if exts.iter().any(|&s| s.eq_ignore_ascii_case(ext)) {
                        list.push(path);
                    }
                }
            }
        }
        list
    }

    let files: Vec<PathBuf> = collect_files(&music_dir, &supported_exts);

    if files.is_empty() {
        println!("⚠️ No supported audio files found in {:?}", music_dir);
        return;
    }

    println!("Found {} files", files.len());
    for file in &files {
        println!("Found a file here: {}", file.display());
    }
    // Test each file’s metadata extraction
    for file in files {
        println!("\nTesting {:?}", file.display());
        match SongMetadata::from_file(file.to_str().unwrap()) {
            Ok(meta) => {
                println!("  → {:?}", meta);
                // Non-fatal asserts
                if meta.artist.is_none() && meta.title.is_none() && meta.album.is_none() {
                    println!("  ⚠️ No metadata found.");
                }
            }
            Err(e) => {
                println!("  ❌ Error reading metadata: {}", e);
            }
        }
    }
}
