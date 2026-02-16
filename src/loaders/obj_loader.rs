//! Defines a file loader for `.obj` files.

use crate::files::get_file_size::get_file_size;
use crate::loaders::object_loader::ObjectLoader;
use crate::models::object_metadata::ObjectMetadata;
use color_eyre::eyre::Report;
use core::f32;
use obj::{Obj, TexturedVertex, load_obj};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::File;
use std::io::BufReader;

// ===== Helper functions ======================================================

/// Computes the axis-aligned bounding box for a set of vertices.
/// Parallelized using Rayon for improved performance on large vertex sets.
fn compute_bounding_box(vertices: &[TexturedVertex]) -> ((f32, f32, f32), (f32, f32, f32)) {
    vertices
        // Parallelize the computation of min and max for each vertex
        .par_iter()
        // Map each vertex to its position and initialize min and max values
        .map(|v| (v.position.into(), v.position.into()))
        // Reduce the results to find the overall min and max
        .reduce(
            || {
                (
                    (f32::MAX, f32::MAX, f32::MAX),
                    (f32::MIN, f32::MIN, f32::MIN),
                )
            },
            |(min1, max1), (min2, max2)| {
                (
                    (min1.0.min(min2.0), min1.1.min(min2.1), min1.2.min(min2.2)),
                    (max1.0.max(max2.0), max1.1.max(max2.1), max1.2.max(max2.2)),
                )
            },
        )
}

// ===== Definition ============================================================

pub struct ObjLoader;

/// A file loader for `.obj` files.
impl ObjectLoader for ObjLoader {
    type Output = Obj<TexturedVertex>;

    /// Gets metadata on a particular 3D object file.
    ///
    /// # Errors
    /// - If the file cannot be read or parsed correctly.
    fn get_metadata(object: &Self::Output, file_path: &str) -> ObjectMetadata {
        ObjectMetadata {
            vertex_count: object.vertices.len(),
            bounding_box: compute_bounding_box(&object.vertices),
            face_count: object.indices.len() / 3, // Assuming triangles
            file_size: get_file_size(file_path),
            file_name: file_path.to_string(),
        }
    }

    /// Loads a 3D object from the specified file path.
    fn load(file_path: &str) -> Result<Self::Output, Report> {
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let obj: Obj<TexturedVertex> = load_obj(reader)?;

        Ok(obj)
    }
}
