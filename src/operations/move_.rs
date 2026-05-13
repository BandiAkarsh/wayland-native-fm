//! File move operations

use crate::operations::{CopyOptions, ProgressTracker, Result};
use std::path::Path;

/// Move a file or directory
pub async fn move_file(
    _src: &Path,
    _dst: &Path,
    _options: &CopyOptions,
    _progress: Option<&dyn ProgressTracker>,
) -> Result<()> {
    Ok(())
}
