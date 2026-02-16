//! Validates that a proper file location is provided.

use crate::models::supported_file_extensions::SupportedFileExtensions;
use color_eyre::eyre::Report;
use std::path::Path;

// ===== Helper functions ======================================================

/// Retrieve the file extension from the provided file path.
fn get_file_extension(file_path: &str) -> Option<String> {
    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_lowercase)
}

/// Validates that a given file path exists.
fn validate_file_exists(path: &Path) -> Result<(), Report> {
    if !path.exists() {
        return Err(Report::msg(format!(
            "The file at path '{}' does not exist.",
            path.display()
        )));
    }

    Ok(())
}

/// Validates that the provided path is not a directory.
fn validate_file_is_not_directory(path: &Path) -> Result<(), Report> {
    if path.is_dir() {
        return Err(Report::msg(format!(
            "The path '{}' is a directory, not a file.",
            path.display()
        )));
    }

    Ok(())
}

/// Validates that the file has a supported extension.
fn validate_supported_extension(file_path: &str) -> Result<(), Report> {
    let file_extension: String = get_file_extension(file_path).ok_or_else(|| {
        Report::msg(format!(
            "The file at path '{file_path}' does not have a valid extension."
        ))
    })?;

    if SupportedFileExtensions::from_string(file_extension.as_str()).is_none() {
        return Err(Report::msg(format!(
            "The file extension '.{file_extension}' is not supported."
        )));
    }

    Ok(())
}

// ===== Definition ============================================================

/// Validates that a proper file location is provided.
///
/// # Errors
/// - If the file does not exist at the provided path.
/// - If the provided path is a directory instead of a file.
/// - If the file extension is not supported.
pub fn validate_file(file_path: &str) -> Result<(), Report> {
    let path: &Path = Path::new(file_path);

    validate_file_exists(path)?;
    validate_file_is_not_directory(path)?;
    validate_supported_extension(file_path)?;

    Ok(())
}
