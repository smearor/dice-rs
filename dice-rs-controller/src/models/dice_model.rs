/// Vertex data for a 3D dice model.
///
/// Contains all per-vertex attributes needed for OpenGL rendering:
/// positions, normals, texture coordinates, face identifiers, and
/// an index buffer. Also carries metadata used by the fragment shader
/// to select the correct edge detection and number rendering logic.
pub struct DiceModel {
    /// Flat array of vertex positions, 3 floats per vertex (x, y, z).
    pub positions: Vec<f32>,
    /// Flat array of vertex normals, 3 floats per vertex (nx, ny, nz).
    /// All vertices of a face share the same normal for flat shading.
    pub normals: Vec<f32>,
    /// Flat array of texture coordinates, 2 floats per vertex (u, v).
    /// UV layout depends on the face shape — triangle, quad, kite, or pentagon.
    pub uvs: Vec<f32>,
    /// Flat array of face identifiers, 1 float per vertex.
    /// Each vertex carries the face number it belongs to (1-indexed).
    /// Used by the shader to render the correct pip pattern or digit.
    pub face_ids: Vec<f32>,
    /// Index buffer for indexed drawing, referencing vertices by position.
    /// Each group of 3 indices forms one triangle.
    pub indices: Vec<u32>,
    /// Controls shader edge detection and number positioning.
    /// 0 = triangle, 1 = quad, 2 = pentagon, 3 = kite.
    pub face_shape: i32,
    /// Whether this die is a D10X (tens die showing 00, 10, 20, ..., 90).
    /// When true, the shader renders two-digit values instead of single digits.
    pub is_d10x: bool,
}

/// Trait for 3D dice geometry generation.
/// Each die type implements this to provide vertex data for OpenGL rendering.
pub trait DiceModelTrait {
    /// Build the vertex data for this die type.
    fn model(&self) -> DiceModel;
}
