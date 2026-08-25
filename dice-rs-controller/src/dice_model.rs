/// Vertex data for a 3D dice model.
///
/// Currently provides a hardcoded cube (D6) with positions, normals, UVs,
/// face IDs, and indices. Each face has 4 vertices (2 triangles) with
/// outward-facing normals. UVs map 0..1 per face for procedural pip
/// rendering. Face IDs encode the die value (1-6) shown on each face.
pub struct DiceModel {
    /// Vertex positions: [x, y, z] per vertex.
    pub positions: Vec<f32>,
    /// Vertex normals: [x, y, z] per vertex.
    pub normals: Vec<f32>,
    /// UV coordinates: [u, v] per vertex (0..1 per face).
    pub uvs: Vec<f32>,
    /// Face IDs (die value 1-6) per vertex.
    pub face_ids: Vec<f32>,
    /// Triangle indices into the vertex arrays.
    pub indices: Vec<u32>,
}

impl DiceModel {
    /// Create a D6 cube model with per-face normals, UVs, and face IDs.
    ///
    /// The cube spans from -0.5 to +0.5 on each axis. Each face has
    /// 4 vertices (2 triangles) with a distinct normal, so lighting
    /// produces sharp edges between faces. Opposite faces sum to 7
    /// (standard die convention): -X=1/+X=6, +Z=2/-Z=5, +Y=3/-Y=4.
    pub fn d6() -> Self {
        // 6 faces × 4 vertices = 24 vertices, 6 faces × 6 indices = 36 indices
        // Each face: ([4 corners], normal, face_value)
        let cube_faces: [([[f32; 3]; 4], [f32; 3], f32); 6] = [
            // +X face → 6
            ([[0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5], [0.5, -0.5, 0.5]], [1.0, 0.0, 0.0], 6.0),
            // -X face → 1
            ([[-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, -0.5]], [-1.0, 0.0, 0.0], 1.0),
            // +Y face (top) → 3
            ([[-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]], [0.0, 1.0, 0.0], 3.0),
            // -Y face (bottom) → 4
            ([[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]], [0.0, -1.0, 0.0], 4.0),
            // +Z face → 2
            ([[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]], [0.0, 0.0, 1.0], 2.0),
            // -Z face → 5
            ([[0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]], [0.0, 0.0, -1.0], 5.0),
        ];

        // UV mapping per face corner (0..1 consistent for all faces)
        let face_uvs: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

        let mut positions = Vec::with_capacity(24 * 3);
        let mut normals_out = Vec::with_capacity(24 * 3);
        let mut uvs = Vec::with_capacity(24 * 2);
        let mut face_ids = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        for (corners, normal, face_value) in &cube_faces {
            let base = (positions.len() / 3) as u32;
            for (i, corner) in corners.iter().enumerate() {
                positions.extend_from_slice(corner);
                normals_out.extend_from_slice(normal);
                uvs.extend_from_slice(&face_uvs[i]);
                face_ids.push(*face_value);
            }
            // Two triangles: (0,1,2) and (0,2,3)
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        Self {
            positions,
            normals: normals_out,
            uvs,
            face_ids,
            indices,
        }
    }
}
