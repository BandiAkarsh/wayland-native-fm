//! File delete operations

use std::path::Path;
use crate::operations::{Result, DeleteOptions, ProgressTracker};

/// Delete a file
pub async fn delete_file(
    _path: &Path,
    _options: &DeleteOptions,
    _progress: Option<&dyn ProgressTracker>,
) -> Result<()> {
    Ok(())
}