use glam::Vec3;

use crate::models::DiceModel;
use crate::models::DiceModelTrait;
use crate::models::add_pent_face;

/// D12 dodecahedron model with 12 pentagonal faces.
///
/// Uses golden ratio construction. Face values 1-12, opposite faces sum to 13.
pub struct D12Model;

impl DiceModelTrait for D12Model {
    fn model(&self) -> DiceModel {
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let inv_phi = 1.0 / phi;
        let norm = 3.0_f32.sqrt();
        let s = 1.0 / norm;
        let p = phi / norm;
        let ip = inv_phi / norm;

        // 20 vertices of a dodecahedron
        let v: [[f32; 3]; 20] = [
            [s, s, s],      //  0
            [s, s, -s],     //  1
            [s, -s, s],     //  2
            [s, -s, -s],    //  3
            [-s, s, s],     //  4
            [-s, s, -s],    //  5
            [-s, -s, s],    //  6
            [-s, -s, -s],   //  7
            [0.0, ip, p],   //  8
            [0.0, ip, -p],  //  9
            [0.0, -ip, p],  // 10
            [0.0, -ip, -p], // 11
            [ip, p, 0.0],   // 12
            [ip, -p, 0.0],  // 13
            [-ip, p, 0.0],  // 14
            [-ip, -p, 0.0], // 15
            [p, 0.0, ip],   // 16
            [p, 0.0, -ip],  // 17
            [-p, 0.0, ip],  // 18
            [-p, 0.0, -ip], // 19
        ];

        // 12 pentagonal faces — each defined by 5 vertex indices
        // Opposite pairs: 1↔12, 2↔11, 3↔10, 4↔9, 5↔8, 6↔7
        let faces: [([usize; 5], f32); 12] = [
            ([0, 8, 4, 14, 12], 1.0),
            ([0, 12, 1, 17, 16], 2.0),
            ([0, 16, 2, 10, 8], 3.0),
            ([1, 12, 14, 5, 9], 4.0),
            ([1, 9, 11, 3, 17], 5.0),
            ([2, 16, 17, 3, 13], 6.0),
            ([2, 13, 15, 6, 10], 7.0),
            ([3, 11, 7, 15, 13], 8.0),
            ([4, 8, 10, 6, 18], 9.0),
            ([4, 18, 19, 5, 14], 10.0),
            ([5, 19, 7, 11, 9], 11.0),
            ([6, 15, 7, 19, 18], 12.0),
        ];

        let mut positions = Vec::with_capacity(72 * 3);
        let mut normals = Vec::with_capacity(72 * 3);
        let mut uvs = Vec::with_capacity(72 * 2);
        let mut face_ids = Vec::with_capacity(72);
        let mut indices = Vec::with_capacity(36 * 3);

        for (idxs, fid) in faces {
            let center = centroid(&[v[idxs[0]], v[idxs[1]], v[idxs[2]], v[idxs[3]], v[idxs[4]]]);
            let corners = [v[idxs[0]], v[idxs[1]], v[idxs[2]], v[idxs[3]], v[idxs[4]]];
            let n = face_normal(&corners);
            add_pent_face(&mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices, center, corners, n, fid);
        }

        DiceModel {
            positions,
            normals,
            uvs,
            face_ids,
            indices,
            face_shape: 2,
            is_d10x: false,
        }
    }
}

fn centroid(pts: &[[f32; 3]; 5]) -> [f32; 3] {
    let mut c = Vec3::ZERO;
    for p in pts {
        c += Vec3::from(*p);
    }
    (c / 5.0).into()
}

fn face_normal(corners: &[[f32; 3]; 5]) -> [f32; 3] {
    let e0 = Vec3::from(corners[1]) - Vec3::from(corners[0]);
    let e1 = Vec3::from(corners[2]) - Vec3::from(corners[0]);
    e0.cross(e1).normalize().into()
}
