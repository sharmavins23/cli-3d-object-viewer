//! Helper for getting a file's size.

use bytesize::ByteSize;
use std::fs::metadata;

// ===== Definition ============================================================

/// Get a file's size.
#[must_use]
pub fn get_file_size(file_path: &str) -> ByteSize {
    metadata(file_path).map_or_else(|_| ByteSize::b(0), |m| ByteSize::b(m.len()))
}
