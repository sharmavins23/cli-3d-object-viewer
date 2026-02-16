use clap::Subcommand;

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
