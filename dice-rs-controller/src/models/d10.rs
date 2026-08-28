use glam::Vec3;

use crate::models::DiceModel;
use crate::models::DiceModelTrait;
use crate::models::add_kite_face;

/// D10 pentagonal trapezohedron model with 10 kite-shaped faces.
///
/// Each face is split into 2 triangles. Face values 1-10,
/// opposite faces sum to 11.
pub struct D10Model;

impl DiceModelTrait for D10Model {
    fn model(&self) -> DiceModel {
        // A pentagonal trapezohedron has 12 vertices:
        // 2 polar vertices (top/bottom) and 10 equatorial vertices in two pentagons.
        let h = 0.7; // polar height
        // For planar kite faces: r_belly = r_wing * cos(36°).
        // Even indices are "wing" vertices (shared between two same-pole kites) → larger radius.
        // Odd indices are "belly" vertices (shared between upper and lower kites) → smaller radius.
        let r_wing = 0.6;
        let r_belly = r_wing * 0.809_017; // cos(36°)

        // Upper polar vertex
        let top = [0.0, h, 0.0];
        // Lower polar vertex
        let bottom = [0.0, -h, 0.0];

        // 10 equatorial vertices — all at y=0, alternating between two radii.
        // Even indices at r_wing, odd indices at r_belly.
        let mut eq = [[0.0; 3]; 10];
        for (i, eq_i) in eq.iter_mut().enumerate() {
            let angle = std::f32::consts::TAU * (i as f32) / 10.0;
            let r = if i % 2 == 0 { r_wing } else { r_belly };
            *eq_i = [r * angle.cos(), 0.0, r * angle.sin()];
        }

        // 10 faces: 5 upper kites + 5 lower kites
        // Upper kite i: top, eq[2i], eq[2i+1], eq[2i+2] (4 vertices, 2 triangles)
        // Lower kite i: bottom, eq[2i+1], eq[2i+2], eq[2i+3] (4 vertices, 2 triangles)
        // Face values: upper 1-5, lower 6-10 (opposite pairs sum to 11)
        let mut positions = Vec::with_capacity(40 * 3);
        let mut normals = Vec::with_capacity(40 * 3);
        let mut uvs = Vec::with_capacity(40 * 2);
        let mut face_ids = Vec::with_capacity(40);
        let mut indices = Vec::with_capacity(60);

        for i in 0..5 {
            let i0 = 2 * i;
            let i1 = (2 * i + 1) % 10;
            let i2 = (2 * i + 2) % 10;
            let fid = (i + 1) as f32;

            // Upper kite: top, eq[i0], eq[i1], eq[i2]
            let n = tri_normal(top, eq[i0], eq[i1]);
            add_kite_face(&mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices, top, eq[i0], eq[i1], eq[i2], n, fid);
        }

        for i in 0..5 {
            let i0 = (2 * i + 1) % 10;
            let i1 = (2 * i + 2) % 10;
            let i2 = (2 * i + 3) % 10;
            // Opposite face: 11 - (i+1) = 10-i
            let fid = (10 - i) as f32;

            // Lower kite: bottom, eq[i2], eq[i1], eq[i0]
            let n = tri_normal(bottom, eq[i2], eq[i1]);
            add_kite_face(&mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices, bottom, eq[i2], eq[i1], eq[i0], n, fid);
        }

        DiceModel {
            positions,
            normals,
            uvs,
            face_ids,
            indices,
            face_shape: 3,
            is_d10x: false,
        }
    }
}

fn tri_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let e0 = Vec3::from(b) - Vec3::from(a);
    let e1 = Vec3::from(c) - Vec3::from(a);
    e0.cross(e1).normalize().into()
}
