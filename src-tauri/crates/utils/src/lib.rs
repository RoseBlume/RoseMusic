use std::sync::LazyLock;
use std::env;
pub static USERNAME: LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }
});


pub static MUSIC_FOLDER_PATH: LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "android")] {
        "/storage/emulated/0/Music"
        // "/storage/self/primary/Music".to_string()
        //"/sdcard/Music".to_string()
    }

    #[cfg(target_os = "windows")] {
        format!("C:\\Users\\{}\\Music", *USERNAME)
    }
    #[cfg(target_os = "linux")] {
        format!("/home/{}/Music", *USERNAME)
    }
});

pub static SCANFILE_PATH: LazyLock<String> = std::sync::LazyLock::new(|| {
    // On Windows, use APPDATA\RosaryMusic\scan.json
    #[cfg(target_os = "windows")] {
        format!("C:\\Users\\{}\\AppData\\Local\\Rosary Music\\scan.json", *USERNAME)
    }
    // On macOS, use ~/Library/Application Support/RosaryMusic/scan.json
    #[cfg(target_os = "macos")] {
    format!("/Users/{}/Library/Application Support/RosaryMusic/scan.json", *USERNAME)
    }
    // On Linux, use ~/.config/RosaryMusic/scan.json
    
    #[cfg(target_os = "linux")] {
        format!("/home/{}/.config/Rosary Music/scan.json", *USERNAME)
    }
    // Switch to apps private directory in the future
    #[cfg(target_os = "android")] {
        String::from("/storage/emulated/0/Documents/scan.json")

    }
});

