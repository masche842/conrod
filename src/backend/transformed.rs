use super::types::{Scalar};

/// Implemented by contexts that can transform.
pub trait Transformed: Sized {
    /// Translate x an y in local coordinates.
    fn trans(self, x: Scalar, y: Scalar) -> Self;

    /// Scales in local coordinates.
    fn scale(self, sx: Scalar, sy: Scalar) -> Self;
}