//! Command Line Interface (CLI) structure for argument parsing.

use crate::models::object_viewer_action::ObjectViewerAction;
use clap::Parser;

// ===== Definition ============================================================

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct CLIArguments {
    /// Performed action
    #[command(subcommand)]
    pub action: ObjectViewerAction,
}
