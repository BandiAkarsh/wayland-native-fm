//! File operations module

pub mod copy;
pub mod move_;
pub mod delete;

pub use self::copy::*;
pub use self::delete::*;
pub use self::move_::*;

pub type Result<T> = std::result::Result<T, crate::error::FileManagerError>;

/// Copy options
#[derive(Default)]
pub struct CopyOptions {
    pub overwrite: bool,
}

/// Operation progress
#[derive(Default)]
pub struct OperationProgress {
    pub current_file: Option<std::path::PathBuf>,
    pub total_bytes: Option<u64>,
    pub processed_bytes: u64,
    pub files_processed: usize,
    pub total_files: usize,
}

impl OperationProgress {
    pub fn new(total: usize) -> Self {
        Self {
            current_file: None,
            total_bytes: None,
            processed_bytes: 0,
            files_processed: 0,
            total_files: total,
        }
    }
}

/// Progress tracker trait
pub trait ProgressTracker {
    fn report_progress(&self, progress: &OperationProgress);
}

/// Batch result
#[derive(Default)]
pub struct BatchResult<T> {
    pub items: Vec<T>,
}

/// Delete options
#[derive(Default)]
pub struct DeleteOptions {
    pub recursive: bool,
    pub force: bool,
}