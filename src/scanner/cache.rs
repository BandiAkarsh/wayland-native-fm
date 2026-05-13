//! Scanner cache stub

use std::time::Duration;

/// Directory cache stub
#[allow(dead_code)]
pub struct SharedDirectoryCache {
    ttl: Duration,
}

impl SharedDirectoryCache {
    #[allow(dead_code)]
    pub fn new(_capacity: usize, ttl: Duration) -> Self {
        Self { ttl }
    }
}
