//! The main entry point of the application.

use clap::Parser;
use cli_3d_object_viewer::models::{
    cli_arguments::CLIArguments, object_viewer_action::ObjectViewerAction,
};

// ===== Driver code ===========================================================

/// The main entry point of the application.
fn main() {
    let cli_arguments: CLIArguments = CLIArguments::parse();

    println!("Parsed CLI arguments: {cli_arguments:#?}");

    match cli_arguments.action {
        ObjectViewerAction::Inspect { file } => {
            println!("Inspecting metadata of the 3D object from file: {file}");
        }
        ObjectViewerAction::Render { file } => {
            println!("Rendering the 3D object from file: {file}");
        }
    }
}
