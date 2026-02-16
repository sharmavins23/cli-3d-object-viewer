//! Helper to extract the file extension from a given file path.

use crate::models::supported_file_extensions::SupportedFileExtensions;
use std::ffi::OsStr;
use std::path::Path;

// ===== Definition ============================================================

/// Extracts the file extension from the provided file path and checks if it is supported.
#[must_use]
pub fn extract_extension(file_path: &str) -> Option<SupportedFileExtensions> {
    Path::new(file_path)
        .extension()
        .and_then(OsStr::to_str)
        .and_then(SupportedFileExtensions::from_string)
}
