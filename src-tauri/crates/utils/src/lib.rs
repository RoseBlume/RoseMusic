use std::sync::LazyLock;

#[cfg(target_os = "windows")]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
});

#[cfg(not(target_os = "windows"))]
pub static USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
});

pub static MUSIC_FOLDER_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Music".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\Music", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/Music", *USERNAME)
    }
});

pub static SCANFILE_PATH: LazyLock<String> = LazyLock::new(|| {
    #[cfg(target_os = "windows")]
    {
        format!("C:\\Users\\{}\\AppData\\Local\\Rosary Music\\scan.json", *USERNAME)
    }
    #[cfg(target_os = "macos")]
    {
        format!("/Users/{}/Library/Application Support/RosaryMusic/scan.json", *USERNAME)
    }
    #[cfg(target_os = "linux")]
    {
        format!("/home/{}/.config/Rosary Music/scan.json", *USERNAME)
    }
    #[cfg(target_os = "android")]
    {
        "/storage/emulated/0/Documents/scan.json".to_string()
    }
});