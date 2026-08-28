use glam::Vec3;

use crate::models::add_tri_face;
use crate::models::DiceModel;
use crate::models::DiceModelTrait;

/// D4 tetrahedron model with 4 triangular faces.
///
/// Vertices are at the corners of a regular tetrahedron inscribed in a
/// unit sphere. Face values 1-4, opposite vertices sum to 5.
pub struct D4Model;

impl DiceModelTrait for D4Model {
    fn model(&self) -> DiceModel {
        let s = 0.57735027; // 1/sqrt(3) — vertices on unit sphere
        let v = [
            [s, s, s],
            [s, -s, -s],
            [-s, s, -s],
            [-s, -s, s],
        ];

        // Faces: (vertex indices, face_id) — outward-facing winding
        // Opposite face pairs: 1↔4, 2↔3
        let faces: [(usize, usize, usize, f32); 4] = [
            (0, 1, 2, 1.0), // face opposite to v3
            (0, 3, 1, 2.0), // face opposite to v2
            (0, 2, 3, 3.0), // face opposite to v1
            (1, 3, 2, 4.0), // face opposite to v0
        ];

        let mut positions = Vec::with_capacity(12 * 3);
        let mut normals = Vec::with_capacity(12 * 3);
        let mut uvs = Vec::with_capacity(12 * 2);
        let mut face_ids = Vec::with_capacity(12);
        let mut indices = Vec::with_capacity(12);

        for (i0, i1, i2, fid) in faces {
            let e01 = Vec3::from(v[i1]) - Vec3::from(v[i0]);
            let e02 = Vec3::from(v[i2]) - Vec3::from(v[i0]);
            let n: [f32; 3] = e01.cross(e02).normalize().into();
            add_tri_face(
                &mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices,
                v[i0], v[i1], v[i2], n, fid,
            );
        }

        DiceModel { positions, normals, uvs, face_ids, indices, face_shape: 0, is_d10x: false }
    }
}
