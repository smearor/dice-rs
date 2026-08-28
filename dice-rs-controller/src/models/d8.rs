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
            let e01 = [v[i1][0] - v[i0][0], v[i1][1] - v[i0][1], v[i1][2] - v[i0][2]];
            let e02 = [v[i2][0] - v[i0][0], v[i2][1] - v[i0][1], v[i2][2] - v[i0][2]];
            let n = normalize(cross(e01, e02));
            add_tri_face(
                &mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices,
                v[i0], v[i1], v[i2], n, fid,
            );
        }

        DiceModel { positions, normals, uvs, face_ids, indices, face_shape: 0, is_d10x: false }
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-10 {
        return [0.0, 1.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}
