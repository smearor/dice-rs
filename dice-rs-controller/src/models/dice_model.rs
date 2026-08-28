/// Vertex data for a 3D dice model.
pub struct DiceModel {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub face_ids: Vec<f32>,
    pub indices: Vec<u32>,
    /// 0 = triangle/kite, 1 = quad, 2 = pentagon, 3 = kite — controls shader edge detection.
    pub face_shape: i32,
    /// True for D10X (face values shown as 00, 10, ..., 90 instead of 1-10).
    pub is_d10x: bool,
}

/// Trait for 3D dice geometry generation.
/// Each die type implements this to provide vertex data for OpenGL rendering.
pub trait DiceModelTrait {
    /// Build the vertex data for this die type.
    fn model(&self) -> DiceModel;
}
