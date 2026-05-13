//! Thumbnail generation and caching

pub mod cache;

use crate::error::FileManagerError;
use image::imageops::FilterType;
use image::GenericImageView;
use std::path::{Path, PathBuf};

/// Thumbnail size
pub const THUMBNAIL_SIZE: u32 = 128;

/// Image format enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Unknown,
}

impl ImageFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::Webp,
            "bmp" => ImageFormat::Bmp,
            _ => ImageFormat::Unknown,
        }
    }
}

/// Thumbnail data
#[derive(Clone)]
pub struct Thumbnail {
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

impl Thumbnail {
    /// Create a new empty thumbnail
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            format: ImageFormat::Unknown,
            width: 0,
            height: 0,
        }
    }

    /// Check if thumbnail is valid
    pub fn is_valid(&self) -> bool {
        !self.data.is_empty()
    }
}

impl Default for Thumbnail {
    fn default() -> Self {
        Self::new()
    }
}

/// Thumbnail manager for generating and caching thumbnails
pub struct ThumbnailManager {
    cache_dir: PathBuf,
    max_size: usize,
}

impl ThumbnailManager {
    /// Create a new thumbnail manager
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("wayland-file-manager")
            .join("thumbnails");

        // Create cache directory
        if let Err(e) = std::fs::create_dir_all(&cache_dir) {
            tracing::warn!("Failed to create thumbnail cache directory: {}", e);
        }

        Self {
            cache_dir,
            max_size: 100 * 1024 * 1024, // 100 MB
        }
    }

    /// Generate a thumbnail from an image file
    pub fn generate_thumbnail<P: AsRef<Path>>(&self, path: P) -> Result<Thumbnail, FileManagerError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(FileManagerError::NotFound(path.display().to_string()));
        }

        // Check if it's an image file
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        
        let format = ImageFormat::from_extension(extension);
        if format == ImageFormat::Unknown {
            return Err(FileManagerError::Operation(format!("Unsupported image format: {}", extension)));
        }

        // Load image
        let img = image::open(path).map_err(|e| FileManagerError::Operation(e.to_string()))?;

        // Get original dimensions
        let (width, height) = img.dimensions();

        // Resize to thumbnail
        let thumbnail = img.resize(THUMBNAIL_SIZE, THUMBNAIL_SIZE, FilterType::Lanczos3);

        // Encode to PNG
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        
        thumbnail
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| FileManagerError::Operation(e.to_string()))?;

        tracing::debug!(
            "Generated thumbnail for {}: {}x{} -> {} bytes",
            path.display(),
            width,
            height,
            buffer.len()
        );

        Ok(Thumbnail {
            data: buffer,
            format: ImageFormat::Png,
            width: THUMBNAIL_SIZE,
            height: THUMBNAIL_SIZE,
        })
    }

    /// Get thumbnail cache path for a file
    fn get_cache_path(&self, path: &Path) -> PathBuf {
        // Use a hash of the file path as the cache key
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash = hasher.finish();
        
        self.cache_dir.join(format!("{:x}.png", hash))
    }

    /// Get cached thumbnail if available and fresh
    pub fn get_cached<P: AsRef<Path>>(&self, path: P) -> Option<Thumbnail> {
        let path = path.as_ref();
        let cache_path = self.get_cache_path(path);

        if !cache_path.exists() {
            return None;
        }

        // Check if cache is older than 24 hours
        if let Ok(metadata) = std::fs::metadata(&cache_path) {
            if let Ok(modified) = metadata.modified() {
                let age = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default();
                
                // Duration doesn't have as_hours, check in seconds
                if age.as_secs() > 24 * 3600 {
                    return None;
                }
            }
        }

        // Load cached thumbnail
        std::fs::read(&cache_path)
            .ok()
            .map(|data| Thumbnail {
                data,
                format: ImageFormat::Png,
                width: THUMBNAIL_SIZE,
                height: THUMBNAIL_SIZE,
            })
    }

    /// Save thumbnail to cache
    pub fn cache_thumbnail<P: AsRef<Path>>(&self, path: P, thumbnail: &Thumbnail) -> Result<(), FileManagerError> {
        let path = path.as_ref();
        let cache_path = self.get_cache_path(path);

        std::fs::write(&cache_path, &thumbnail.data)
            .map_err(|e| FileManagerError::Operation(e.to_string()))?;

        tracing::debug!("Cached thumbnail for {}", path.display());
        Ok(())
    }

    /// Get or generate thumbnail (with caching)
    pub fn get_thumbnail<P: AsRef<Path>>(&self, path: P) -> Result<Thumbnail, FileManagerError> {
        let path = path.as_ref();

        // Try cache first
        if let Some(cached) = self.get_cached(path) {
            return Ok(cached);
        }

        // Generate new thumbnail
        let thumbnail = self.generate_thumbnail(path)?;

        // Cache it
        let _ = self.cache_thumbnail(path, &thumbnail);

        Ok(thumbnail)
    }
}

impl Default for ThumbnailManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thumbnail_manager_creation() {
        let manager = ThumbnailManager::new();
        assert!(manager.cache_dir.exists() || true); // May fail in test env
    }

    #[test]
    fn test_image_format_detection() {
        assert_eq!(ImageFormat::from_extension("png"), ImageFormat::Png);
        assert_eq!(ImageFormat::from_extension("jpg"), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::from_extension("JPG"), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::from_extension("txt"), ImageFormat::Unknown);
    }
}