//! Defines a file loader for `.obj` files.

use crate::loaders::object_loader::ObjectLoader;
use color_eyre::eyre::Report;
use obj::{Obj, TexturedVertex, load_obj};
use std::fs::File;
use std::io::BufReader;

// ===== Definition ============================================================

pub struct ObjLoader;

/// A file loader for `.obj` files.
impl ObjectLoader for ObjLoader {
    type Output = Obj<TexturedVertex>;

    /// Loads a 3D object from the specified file path.
    fn load(file_path: &str) -> Result<Self::Output, Report> {
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let obj: Obj<TexturedVertex> = load_obj(reader)?;

        Ok(obj)
    }
}
