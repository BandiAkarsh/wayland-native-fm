//! File copy operations with progress tracking

use crate::error::FileManagerError;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Copy options
#[derive(Default)]
pub struct CopyOptions {
    pub overwrite: bool,
    pub buffer_size: usize,
}

impl CopyOptions {
    pub fn new() -> Self {
        Self {
            overwrite: false,
            buffer_size: 64 * 1024, // 64 KB default buffer
        }
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
}

/// Copy file with progress callback
pub async fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
    options: CopyOptions,
) -> Result<u64, FileManagerError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(FileManagerError::NotFound(src.display().to_string()));
    }

    if !src.is_file() {
        return Err(FileManagerError::NotFile(src.display().to_string()));
    }

    if dst.exists() && !options.overwrite {
        return Err(FileManagerError::AlreadyExists(dst.display().to_string()));
    }

    // Ensure parent directory exists
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }

    // Get file size for progress
    let metadata = fs::metadata(src)?;
    let total_size = metadata.len();
    
    tracing::info!("Copying {} -> {} ({} bytes)", src.display(), dst.display(), total_size);

    // Copy file
    let mut source = fs::File::open(src)?;
    let mut dest = fs::File::create(dst)?;
    
    let mut buffer = vec![0u8; options.buffer_size];
    let mut copied: u64 = 0;
    
    loop {
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        dest.write_all(&buffer[..bytes_read])?;
        copied += bytes_read as u64;
        
        // Progress could be reported here via callback
        if copied % (1024 * 1024) == 0 { // Log every MB
            let progress = (copied as f64 / total_size as f64 * 100.0) as u32;
            tracing::debug!("Copy progress: {}%", progress);
        }
    }

    // Preserve permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(src) {
            let permissions = metadata.permissions();
            if let Err(e) = fs::set_permissions(dst, permissions) {
                tracing::warn!("Failed to preserve permissions: {}", e);
            }
        }
    }

    tracing::info!("Copied {} bytes", copied);
    Ok(copied)
}

/// Copy directory recursively
pub async fn copy_directory<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
    options: CopyOptions,
) -> Result<u64, FileManagerError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(FileManagerError::NotFound(src.display().to_string()));
    }

    if !src.is_dir() {
        return Err(FileManagerError::NotDirectory(src.display().to_string()));
    }

    // Create destination directory
    fs::create_dir_all(dst)?;

    let mut total_copied: u64 = 0;

    for entry in walkdir::WalkDir::new(src)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src).unwrap();
        let dst_path = dst.join(relative_path);

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else if src_path.is_file() {
            let opts = CopyOptions {
                overwrite: options.overwrite,
                buffer_size: options.buffer_size,
            };
            match copy_file(src_path, &dst_path, opts).await {
                Ok(bytes) => total_copied += bytes,
                Err(e) => {
                    tracing::warn!("Failed to copy {}: {}", src_path.display(), e);
                }
            }
        }
    }

    tracing::info!("Directory copy complete: {} bytes", total_copied);
    Ok(total_copied)
}

/// Async copy file using tokio
pub async fn copy_file_async<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> Result<u64, FileManagerError> {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Err(FileManagerError::NotFound(src.display().to_string()));
    }

    // Ensure parent directory exists
    if let Some(parent) = dst.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Copy using tokio
    let mut src_file = tokio::fs::File::open(src).await?;
    let _total_size = src_file.metadata().await?.len();
    
    let mut dst_file = tokio::fs::File::create(dst).await?;
    
    let mut buffer = vec![0u8; 64 * 1024];
    let mut copied: u64 = 0;
    
    loop {
        let bytes_read = src_file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        
        dst_file.write_all(&buffer[..bytes_read]).await?;
        copied += bytes_read as u64;
    }

    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_copy_file() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source.txt");
        let dst = temp_dir.path().join("dest.txt");
        
        // Create source file
        std::fs::write(&src, "Hello, World!").unwrap();
        
        let options = CopyOptions::new();
        let result = copy_file(&src, &dst, options).await;
        
        assert!(result.is_ok());
        assert!(dst.exists());
        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "Hello, World!");
    }
}