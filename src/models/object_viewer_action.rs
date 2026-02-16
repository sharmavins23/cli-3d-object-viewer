//! Defines different actions the 3D object viewer can perform.

use clap::Subcommand;

// ===== Definition ============================================================

/// Defines the different actions that the 3D object viewer can perform.
#[derive(Debug, Subcommand)]
pub enum ObjectViewerAction {
    /// Action to perform: Inspect metadata of the 3D object.
    Inspect {
        /// Path to the 3D object file to be processed.
        #[arg(short, long)]
        file: String,
    },

    /// Action to perform: Render the 3D object.
    Render {
        /// Path to the 3D object file to be processed.
        #[arg(short, long)]
        file: String,
    },
}

// ===== Implementation ========================================================

/// Defines the different actions that the 3D object viewer can perform.
impl ObjectViewerAction {
    /// Retrieves the file path associated with the action, regardless of the
    /// specific action type.
    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::Inspect { file } | Self::Render { file } => file,
        }
    }
}
