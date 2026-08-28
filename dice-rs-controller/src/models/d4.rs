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
