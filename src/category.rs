use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Rust,
    Markdown,
    Image,
    Audio,
    Video,
    Archive,
    Other,
}

pub fn classify(path: &Path) -> Category {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        // Rust
        "rs" => Category::Rust,

        // Markdown
        "md" | "markdown" => Category::Markdown,

        // Image
        "png"
        | "jpg"
        | "jpeg"
        | "gif"
        | "bmp"
        | "tiff"
        | "webp"
        | "svg"
        | "ico" => Category::Image,

        // Audio
        "mp3"
        | "wav"
        | "flac"
        | "aac"
        | "ogg"
        | "m4a"
        | "aiff"
        | "wma" => Category::Audio,

        // Video
        "mp4"
        | "mov"
        | "avi"
        | "mkv"
        | "wmv"
        | "webm"
        | "flv"
        | "mpeg"
        | "mpg" => Category::Video,

        // Archive
        "zip"
        | "tar"
        | "gz"
        | "tgz"
        | "xz"
        | "7z"
        | "rar"
        | "bz2" => Category::Archive,

        // Other
        _ => Category::Other,
    }
}
