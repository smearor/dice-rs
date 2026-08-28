use crate::models::add_quad_face;
use crate::models::DiceModel;
use crate::models::DiceModelTrait;

/// D6 cube model with per-face normals, UVs, and face IDs.
///
/// Opposite faces sum to 7 (standard die convention):
/// -X=1/+X=6, +Z=2/-Z=5, +Y=3/-Y=4.
pub struct D6Model;

impl DiceModelTrait for D6Model {
    fn model(&self) -> DiceModel {
        let cube_faces: [([[f32; 3]; 4], [f32; 3], f32); 6] = [
            ([[0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5], [0.5, -0.5, 0.5]], [1.0, 0.0, 0.0], 6.0),
            ([[-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, -0.5]], [-1.0, 0.0, 0.0], 1.0),
            ([[-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]], [0.0, 1.0, 0.0], 3.0),
            ([[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]], [0.0, -1.0, 0.0], 4.0),
            ([[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]], [0.0, 0.0, 1.0], 2.0),
            ([[0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]], [0.0, 0.0, -1.0], 5.0),
        ];

        let mut positions = Vec::with_capacity(24 * 3);
        let mut normals = Vec::with_capacity(24 * 3);
        let mut uvs = Vec::with_capacity(24 * 2);
        let mut face_ids = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        for (corners, normal, face_value) in &cube_faces {
            add_quad_face(
                &mut positions, &mut normals, &mut uvs, &mut face_ids, &mut indices,
                corners[0], corners[1], corners[2], corners[3], *normal, *face_value,
            );
        }

        DiceModel { positions, normals, uvs, face_ids, indices, face_shape: 1, is_d10x: false }
    }
}
