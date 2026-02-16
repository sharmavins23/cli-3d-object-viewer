// ===== Definition ============================================================

/// Defines the supported file extensions for 3D object files.
#[derive(Debug, PartialEq, Eq)]
pub enum SupportedFileExtensions {
    Obj,
}

// ===== Implementation ========================================================

/// Defines the supported file extensions for 3D object files.
impl SupportedFileExtensions {
    /// Converts a string representation of a file extension into a
    /// `SupportedFileExtensions` enum variant, if it is supported.
    #[must_use]
    pub fn from_string(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "obj" => Some(Self::Obj),
            _ => None,
        }
    }
}
