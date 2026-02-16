//! Enumeration containing all supported 3D object output types.

use obj::{Obj, TexturedVertex};

// ===== Definition ============================================================

/// An enumeration representing a generic 3D object.
pub enum Generic3DObject {
    Obj(Obj<TexturedVertex>),
}
