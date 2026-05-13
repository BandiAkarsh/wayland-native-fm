//! Logging infrastructure for the file manager

use std::path::PathBuf;
use std::sync::OnceLock;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Static storage for the non-blocking guard
/// This prevents the guard from being dropped while the program is running
static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

/// Initialize the logging system
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Get log directory
    let log_dir = get_log_directory();

    // Create log directory if it doesn't exist
    std::fs::create_dir_all(&log_dir)?;

    // Create file appender with daily rotation
    let file_appender =
        RollingFileAppender::new(Rotation::DAILY, &log_dir, "wayland-file-manager.log");

    // Create non-blocking writer
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Store the guard in a static to prevent it from being dropped
    // This is the proper way to handle non-blocking logging guards
    let _ = LOG_GUARD.set(guard);

    // Create env filter - default to info, can be overridden by RUST_LOG
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wayland_file_manager=debug"));

    // Initialize subscriber with both console and file output
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    tracing::info!("Logging initialized. Log directory: {:?}", log_dir);

    Ok(())
}

/// Get the log directory path
fn get_log_directory() -> PathBuf {
    // Check XDG_DATA_HOME first
    if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
        return PathBuf::from(data_home)
            .join("wayland-file-manager")
            .join("logs");
    }

    // Fall back to ~/.local/share
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("wayland-file-manager")
            .join("logs");
    }

    // Last resort: /tmp
    PathBuf::from("/tmp")
        .join("wayland-file-manager")
        .join("logs")
}

/// Get the current log file path
pub fn get_log_file_path() -> PathBuf {
    get_log_directory().join("wayland-file-manager.log")
}
