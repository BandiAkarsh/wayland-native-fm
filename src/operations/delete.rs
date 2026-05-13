//! File delete operations

use crate::operations::{DeleteOptions, ProgressTracker, Result};
use std::path::Path;

/// Delete a file
pub async fn delete_file(
    _path: &Path,
    _options: &DeleteOptions,
    _progress: Option<&dyn ProgressTracker>,
) -> Result<()> {
    Ok(())
}
