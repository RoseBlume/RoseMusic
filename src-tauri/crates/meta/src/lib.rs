use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Default)]
pub struct SongMetadata {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
}

impl SongMetadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_ref = path.as_ref();
        let mut f = File::open(path_ref)?;
        let mut header = [0u8; 12];
        if f.read(&mut header)? < 12 {
            return Ok(Self::default_with_filename(path_ref));
        }
        f.seek(SeekFrom::Start(0))?;

        let mut meta = match &header[0..4] {
            b"RIFF" if &header[8..12] == b"WAVE" => Self::from_wav(&mut f)?,
            b"fLaC" => Self::from_flac(&mut f)?,
            b"ID3\x03" | b"ID3\x04" => Self::from_mp3v2(&mut f)?,
            _ => {
                if let Ok(m) = Self::from_id3v1(&mut f) {
                    m
                } else if let Ok(m) = Self::from_m4a(&mut f) {
                    m
                } else {
                    Self::default()
                }
            }
        };

        // ✅ Automatically assign filename as title if missing
        if meta.title.is_none() {
            meta.title = Some(Self::prettify_filename(path_ref));
        }

        Ok(meta)
    }

    fn default_with_filename(path: &Path) -> Self {
        let mut m = Self::default();
        m.title = Some(Self::prettify_filename(path));
        m
    }

    /// Converts `foo_bar-baz.mp3` → `Foo Bar Baz`
    fn prettify_filename(path: &Path) -> String {
        let file_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");

        file_name
            .replace('_', " ")
            .replace('-', " ")
            .split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    // --- WAV (LIST/INFO) parsing ---
    fn from_wav(f: &mut File) -> io::Result<Self> {
        let mut meta = SongMetadata::default();
        f.seek(SeekFrom::Start(12))?;

        let mut buf = [0u8; 8];
        while f.read(&mut buf)? == 8 {
            let chunk_id = &buf[0..4];
            let chunk_size = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as u64;
            let next = f.seek(SeekFrom::Current(0))? + chunk_size;

            if chunk_id == b"LIST" {
                // Read list type (INFO or others)
                let mut list_type = [0u8; 4];
                f.read_exact(&mut list_type)?;
                if &list_type == b"INFO" {
                    let mut remaining = chunk_size - 4;
                    while remaining >= 8 {
                        let mut sub_header = [0u8; 8];
                        if f.read(&mut sub_header)? != 8 {
                            break;
                        }
                        let sub_id = &sub_header[0..4];
                        let sub_size =
                            u32::from_le_bytes(sub_header[4..8].try_into().unwrap()) as usize;

                        let mut data = vec![0u8; sub_size];
                        f.read_exact(&mut data)?;
                        let text = String::from_utf8_lossy(&data)
                            .trim_matches(char::from(0))
                            .trim()
                            .to_string();

                        match sub_id {
                            b"IART" => meta.artist = Some(text),
                            b"INAM" => meta.title = Some(text),
                            b"IPRD" => meta.album = Some(text),
                            b"IGNR" => meta.genre = Some(text),
                            _ => {}
                        }

                        remaining = remaining.saturating_sub((8 + sub_size) as u64);
                    }
                } else {
                    f.seek(SeekFrom::Start(next))?;
                }
            } else {
                f.seek(SeekFrom::Start(next))?;
            }
        }
        Ok(meta)
    }

    // --- MP3v1 ---
    fn from_id3v1(f: &mut File) -> io::Result<Self> {
        let len = f.seek(SeekFrom::End(0))?;
        if len < 128 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no id3v1"));
        }
        f.seek(SeekFrom::End(-128))?;
        let mut buf = [0u8; 128];
        f.read_exact(&mut buf)?;
        if &buf[0..3] != b"TAG" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no TAG header"));
        }

        let title = trim_id3v1_text(&buf[3..33]);
        let artist = trim_id3v1_text(&buf[33..63]);
        let album = trim_id3v1_text(&buf[63..93]);
        let genre = Some(format!("{}", buf[127]));

        Ok(SongMetadata {
            artist,
            title,
            album,
            genre,
        })
    }

    // --- MP3v2 ---
    fn from_mp3v2(f: &mut File) -> io::Result<Self> {
        let mut header = [0u8; 10];
        f.read_exact(&mut header)?;
        if &header[0..3] != b"ID3" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no id3v2 header"));
        }

        let tag_size = synchsafe_to_u32(&header[6..10]) as usize;
        let mut tag_data = vec![0u8; tag_size];
        f.read_exact(&mut tag_data)?;

        let mut meta = SongMetadata::default();
        let mut i = 0;
        while i + 10 <= tag_data.len() {
            let id = &tag_data[i..i + 4];
            let size = u32::from_be_bytes(tag_data[i + 4..i + 8].try_into().unwrap()) as usize;
            if size == 0 || i + 10 + size > tag_data.len() {
                break;
            }
            let frame = &tag_data[i + 10..i + 10 + size];
            let text = decode_text_frame(frame);

            match id {
                b"TIT2" => meta.title = text,
                b"TPE1" => meta.artist = text,
                b"TALB" => meta.album = text,
                b"TCON" => meta.genre = text,
                _ => {}
            }

            i += 10 + size;
        }

        Ok(meta)
    }

    // --- FLAC (Vorbis comment) ---
    fn from_flac(f: &mut File) -> io::Result<Self> {
        let mut header = [0u8; 4];
        f.read_exact(&mut header)?;
        if &header != b"fLaC" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "not flac"));
        }

        let mut meta = SongMetadata::default();
        loop {
            let mut block_header = [0u8; 4];
            if f.read(&mut block_header)? != 4 {
                break;
            }

            let last_block = (block_header[0] & 0x80) != 0;
            let block_type = block_header[0] & 0x7F;
            let block_len =
                ((block_header[1] as u32) << 16) | ((block_header[2] as u32) << 8) | block_header[3] as u32;

            if block_type == 4 {
                let mut data = vec![0u8; block_len as usize];
                f.read_exact(&mut data)?;
                parse_vorbis_comments(&mut meta, &data);
            } else {
                f.seek(SeekFrom::Current(block_len as i64))?;
            }

            if last_block {
                break;
            }
        }

        Ok(meta)
    }

    // --- M4A (MP4 atoms) ---
    fn from_m4a(f: &mut File) -> io::Result<Self> {
        let mut meta = SongMetadata::default();
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        let mut i = 0;
        while i + 8 <= data.len() {
            let size = u32::from_be_bytes(data[i..i + 4].try_into().unwrap()) as usize;
            if size < 8 || i + size > data.len() {
                break;
            }
            let atom = &data[i + 4..i + 8];
            if atom == b"\xa9nam" {
                meta.title = extract_m4a_text(&data[i + 8..i + size]);
            } else if atom == b"\xa9ART" {
                meta.artist = extract_m4a_text(&data[i + 8..i + size]);
            } else if atom == b"\xa9alb" {
                meta.album = extract_m4a_text(&data[i + 8..i + size]);
            } else if atom == b"\xa9gen" {
                meta.genre = extract_m4a_text(&data[i + 8..i + size]);
            }
            i += size;
        }
        Ok(meta)
    }
}

// --- Shared helpers ---
fn trim_id3v1_text(b: &[u8]) -> Option<String> {
    let binding = String::from_utf8_lossy(b);
    let s = binding.trim_end_matches('\u{0}').trim();
    if s.is_empty() { None } else { Some(s.to_string()) }
}

fn synchsafe_to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32 & 0x7F) << 21)
        | ((bytes[1] as u32 & 0x7F) << 14)
        | ((bytes[2] as u32 & 0x7F) << 7)
        | (bytes[3] as u32 & 0x7F)
}

fn decode_text_frame(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    match data[0] {
        0 => Some(String::from_utf8_lossy(&data[1..]).trim_matches(char::from(0)).to_string()),
        1 => {
            let utf16: Vec<u16> = data[1..]
                .chunks(2)
                .filter_map(|b| if b.len() == 2 { Some(u16::from_be_bytes([b[0], b[1]])) } else { None })
                .collect();
            Some(String::from_utf16_lossy(&utf16).trim_matches(char::from(0)).to_string())
        }
        _ => None,
    }
}

fn parse_vorbis_comments(meta: &mut SongMetadata, data: &[u8]) {
    if data.len() < 8 { return; }
    let vendor_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let mut idx = 4 + vendor_len;
    if idx + 4 > data.len() { return; }
    let count = u32::from_le_bytes(data[idx..idx + 4].try_into().unwrap()) as usize;
    idx += 4;
    for _ in 0..count {
        if idx + 4 > data.len() { break; }
        let len = u32::from_le_bytes(data[idx..idx + 4].try_into().unwrap()) as usize;
        idx += 4;
        if idx + len > data.len() { break; }
        if let Ok(s) = String::from_utf8(data[idx..idx + len].to_vec()) {
            let parts: Vec<_> = s.splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0].to_ascii_lowercase().as_str() {
                    "artist" => meta.artist = Some(parts[1].to_string()),
                    "title" => meta.title = Some(parts[1].to_string()),
                    "album" => meta.album = Some(parts[1].to_string()),
                    "genre" => meta.genre = Some(parts[1].to_string()),
                    _ => {}
                }
            }
        }
        idx += len;
    }
}

fn extract_m4a_text(data: &[u8]) -> Option<String> {
    let mut i = 0;
    while i + 8 <= data.len() {
        let size = u32::from_be_bytes(data[i..i + 4].try_into().unwrap()) as usize;
        if size < 8 || i + size > data.len() {
            break;
        }
        if &data[i + 4..i + 8] == b"data" {
            let text = String::from_utf8_lossy(&data[i + 16..i + size]);
            return Some(text.trim_matches(char::from(0)).to_string());
        }
        i += size;
    }
    None
}
