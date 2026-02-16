//! The main entry point of the application.

use clap::Parser;
use cli_3d_object_viewer::{
    files::{extract_extension::extract_extension, validate_file::validate_file},
    loaders::object_loader::load_3d_object,
    models::{
        cli_arguments::CLIArguments, generic_3d_object::Generic3DObject,
        object_viewer_action::ObjectViewerAction,
        supported_file_extensions::SupportedFileExtensions,
    },
    utils::log,
};

// ===== Driver code ===========================================================

/// The main entry point of the application.
fn main() {
    log::dbg("Starting the CLI 3D object viewer...");
    let cli_arguments: CLIArguments = CLIArguments::parse();

    log::dbg("Extracting file location...");
    let file_path: &str = cli_arguments.action.file();

    log::dbg("Validating provided file location...");
    if let Err(e) = validate_file(file_path) {
        log::err(&format!("File validation failed: {e}"));
        std::process::exit(1);
    }

    log::dbg("Extracting filetype...");
    let extension: SupportedFileExtensions = extract_extension(file_path).unwrap_or_else(|| {
        log::err(&format!(
            "Failed to extract a supported file extension from the provided file path: '{file_path}'"
        ));
        std::process::exit(1);
    });

    log::dbg("Loading 3D object...");
    let _object: Generic3DObject = match load_3d_object(file_path, &extension) {
        Ok(obj) => obj,
        Err(e) => {
            log::err(&format!("Failed to load 3D object: {e}"));
            std::process::exit(1);
        }
    };
}
