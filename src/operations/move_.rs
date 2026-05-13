//! File move operations

use std::path::Path;
use crate::operations::{Result, CopyOptions, ProgressTracker};

/// Move a file or directory
pub async fn move_file(
    _src: &Path,
    _dst: &Path,
    _options: &CopyOptions,
    _progress: Option<&dyn ProgressTracker>,
) -> Result<()> {
    Ok(())
}