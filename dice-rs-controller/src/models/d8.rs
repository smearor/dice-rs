use glam::Vec3;

use crate::models::add_tri_face;
use crate::models::DiceModel;
use crate::models::DiceModelTrait;

/// D8 octahedron model with 8 triangular faces.
///
/// Vertices on the coordinate axes. Face values 1-8, opposite faces sum to 9.
pub struct D8Model;

impl DiceModelTrait for D8Model {
    fn model(&self) -> DiceModel {
        let v = [
            [1.0, 0.0, 0.0],   // 0: +X
            [-1.0, 0.0, 0.0],  // 1: -X
            [0.0, 1.0, 0.0],   // 2: +Y
            [0.0, -1.0, 0.0],  // 3: -Y
            [0.0, 0.0, 1.0],   // 4: +Z
            [0.0, 0.0, -1.0],  // 5: -Z
        ];

        // Faces: (i0, i1, i2, face_id) — 8 triangles, outward-facing
        // Opposite pairs: 1↔8, 2↔7, 3↔6, 4↔5
        let faces: [(usize, usize, usize, f32); 8] = [
            (2, 4, 0, 1.0), // +X +Y +Z
            (2, 0, 5, 2.0), // +X +Y -Z
            (2, 1, 4, 3.0), // -X +Y +Z
            (2, 5, 1, 4.0), // -X +Y -Z
            (3, 0, 4, 5.0), // +X -Y +Z
            (3, 5, 0, 6.0), // +X -Y -Z
            (3, 4, 1, 7.0), // -X -Y +Z
            (3, 1, 5, 8.0), // -X -Y -Z
        ];

        let mut positions = Vec::with_capacity(24 * 3);
        let mut normals = Vec::with_capacity(24 * 3);
        let mut uvs = Vec::with_capacity(24 * 2);
        let mut face_ids = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(24);

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
