
#[tauri::command]
pub fn convert_to_milli(seconds: f64) -> u64 {
    (seconds * 1000.0) as u64
}