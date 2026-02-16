//! Structure to hold metadata about a loaded 3D object.

use bytesize::ByteSize;

// ===== Definition ============================================================

/// A structure to hold metadata about a loaded 3D object.
pub struct ObjectMetadata {
    pub vertex_count: usize,
    pub face_count: usize,
    pub bounding_box: ((f32, f32, f32), (f32, f32, f32)), // (min, max)
    pub file_size: ByteSize,
    pub file_name: String,
}
