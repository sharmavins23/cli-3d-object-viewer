//! Defines a trait for loading 3D objects from files.

use crate::{
    loaders::obj_loader::ObjLoader,
    models::{
        generic_3d_object::Generic3DObject, object_metadata::ObjectMetadata,
        supported_file_extensions::SupportedFileExtensions,
    },
    utils::log,
};
use color_eyre::eyre::Report;

// ===== Trait =================================================================

/// A trait for loading 3D objects from files.
pub trait ObjectLoader {
    /// The type of the loaded 3D object.
    type Output;

    /// Gets metadata on a particular 3D object file.
    ///
    /// # Errors
    /// - If the file cannot be read or parsed correctly.
    fn get_metadata(object: &Self::Output, file_path: &str) -> ObjectMetadata;

    /// Loads a 3D object from the specified file path.
    ///
    /// # Errors
    /// - If the file cannot be read or parsed correctly.
    fn load(file_path: &str) -> Result<Self::Output, Report>;
}

// ===== Definition ============================================================

/// Load a 3D object from the specified filepath using the appropriate loader.
///
/// # Errors
/// - If the file cannot be read or parsed correctly.
pub fn load_3d_object(
    file_path: &str,
    extension: &SupportedFileExtensions,
) -> Result<Generic3DObject, Report> {
    match extension {
        SupportedFileExtensions::Obj => {
            log::dbg("Detected `.obj` file. Loading...");
            let obj = ObjLoader::load(file_path);

            Ok(Generic3DObject::Obj(obj?))
        }
    }
}
