//! The main entry point of the application.

use clap::Parser;
use cli_3d_object_viewer::{
    files::validate_file::validate_file,
    models::{cli_arguments::CLIArguments, object_viewer_action::ObjectViewerAction},
    utils::log,
};

// ===== Driver code ===========================================================

/// The main entry point of the application.
fn main() {
    log::dbg("Starting the CLI 3D object viewer...");
    let cli_arguments: CLIArguments = CLIArguments::parse();

    log::dbg("Validating provided file location...");
    let file_path = match &cli_arguments.action {
        ObjectViewerAction::Inspect { file } => file,
        ObjectViewerAction::Render { file } => file,
    };

    if let Err(e) = validate_file(file_path) {
        log::err(&format!("File validation failed: {e}"));
        std::process::exit(1);
    }

    match cli_arguments.action {
        ObjectViewerAction::Inspect { file } => {
            log::dbg(&format!(
                "Inspecting metadata of the 3D object from file: {file}"
            ));
        }
        ObjectViewerAction::Render { file } => {
            log::dbg(&format!("Rendering the 3D object from file: {file}"));
        }
    }
}
