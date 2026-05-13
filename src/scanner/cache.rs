//! Scanner cache stub

use std::time::Duration;

/// Directory cache stub
pub struct SharedDirectoryCache {
    #[allow(dead_code)]
    ttl: Duration,
}

impl SharedDirectoryCache {
    #[allow(dead_code)]
    pub fn new(_capacity: usize, ttl: Duration) -> Self {
        Self { ttl }
    }
}