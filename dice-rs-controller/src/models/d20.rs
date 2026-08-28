use crate::models::add_tri_face;
use crate::models::DiceModel;
use crate::models::DiceModelTrait;

/// D20 icosahedron model with 20 triangular faces.
///
/// Uses golden ratio construction. Face values 1-20, opposite faces sum to 21.
pub struct D20Model;

impl DiceModelTrait for D20Model {
    fn model(&self) -> DiceModel {
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let norm = (1.0 + phi * phi).sqrt();
        let p = phi / norm;
        let u = 1.0 / norm;

        // 12 vertices of an icosahedron
        let v = [
            [-u, p, 0.0],  //  0
            [u, p, 0.0],   //  1
            [-u, -p, 0.0], //  2
            [u, -p, 0.0],  //  3
            [0.0, -u, p],  //  4
            [0.0, u, p],   //  5
            [0.0, -u, -p], //  6
            [0.0, u, -p],  //  7
            [p, 0.0, -u],  //  8
            [p, 0.0, u],   //  9
            [-p, 0.0, -u], // 10
            [-p, 0.0, u],  // 11
        ];

        // 20 faces — outward-facing winding
        // Opposite pairs: 1↔20, 2↔19, ..., 10↔11
        let faces: [(usize, usize, usize, f32); 20] = [
            (0, 11, 5, 1.0),
            (0, 5, 1, 2.0),
            (0, 1, 7, 3.0),
            (0, 7, 10, 4.0),
            (0, 10, 11, 5.0),
            (1, 5, 9, 6.0),
            (5, 11, 4, 7.0),
            (11, 10, 2, 8.0),
            (10, 7, 6, 9.0),
            (7, 1, 8, 10.0),
            (3, 9, 4, 11.0),
            (3, 4, 2, 12.0),
            (3, 2, 6, 13.0),
            (3, 6, 8, 14.0),
            (3, 8, 9, 15.0),
            (4, 9, 5, 16.0),
            (2, 4, 11, 17.0),
            (6, 2, 10, 18.0),
            (8, 6, 7, 19.0),
            (9, 8, 1, 20.0),
        ];

        let mut positions = Vec::with_capacity(60 * 3);
        let mut normals = Vec::with_capacity(60 * 3);
        let mut uvs = Vec::with_capacity(60 * 2);
        let mut face_ids = Vec::with_capacity(60);
        let mut indices = Vec::with_capacity(60);

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
