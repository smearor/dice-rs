pub mod d4;
pub mod d6;
pub mod d8;
pub mod d10;
pub mod d12;
pub mod d20;
pub mod dice_model;

pub use d4::D4Model;
pub use d6::D6Model;
pub use d8::D8Model;
pub use d10::D10Model;
pub use d12::D12Model;
pub use d20::D20Model;
pub use dice_model::DiceModel;
pub use dice_model::DiceModelTrait;

use dice_rs::model::dice::DiceType;

/// Select the appropriate model implementation for a dice type.
/// Returns the model and whether it is a D10X (tens die).
pub fn model_for_type(dice_type: DiceType) -> (Box<dyn DiceModelTrait>, bool) {
    let is_d10x = dice_type == DiceType::D10X;
    let model: Box<dyn DiceModelTrait> = match dice_type {
        DiceType::D6 => Box::new(D6Model),
        DiceType::D20 => Box::new(D20Model),
        DiceType::D10 | DiceType::D10X => Box::new(D10Model),
        DiceType::D4 => Box::new(D4Model),
        DiceType::D8 => Box::new(D8Model),
        DiceType::D12 => Box::new(D12Model),
    };
    (model, is_d10x)
}

/// Build a triangular face and append its vertices to the mesh buffers.
#[allow(clippy::too_many_arguments)]
fn add_tri_face(
    positions: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    face_ids: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    normal: [f32; 3],
    face_id: f32,
) {
    let base = (positions.len() / 3) as u32;
    let tri_uvs: [[f32; 2]; 3] = [[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]];
    for (i, v) in [v0, v1, v2].iter().enumerate() {
        positions.extend_from_slice(v);
        normals.extend_from_slice(&normal);
        uvs.extend_from_slice(&tri_uvs[i]);
        face_ids.push(face_id);
    }
    indices.extend_from_slice(&[base, base + 1, base + 2]);
}

/// Build a quad face (two triangles) and append its vertices.
#[allow(clippy::too_many_arguments)]
fn add_quad_face(
    positions: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    face_ids: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    v3: [f32; 3],
    normal: [f32; 3],
    face_id: f32,
) {
    let base = (positions.len() / 3) as u32;
    let quad_uvs: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    for (i, v) in [v0, v1, v2, v3].iter().enumerate() {
        positions.extend_from_slice(v);
        normals.extend_from_slice(&normal);
        uvs.extend_from_slice(&quad_uvs[i]);
        face_ids.push(face_id);
    }
    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

/// Build a kite face (two triangles sharing a diagonal) with consistent UVs.
///
/// UV layout maps the kite to a triangle in UV space:
/// v0 (apex) → [0.5, 1.0], v1 (left) → [0.0, 0.0],
/// v2 (center) → [0.5, 0.0], v3 (right) → [1.0, 0.0].
#[allow(clippy::too_many_arguments)]
fn add_kite_face(
    positions: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    face_ids: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    v3: [f32; 3],
    normal: [f32; 3],
    face_id: f32,
) {
    let base = (positions.len() / 3) as u32;
    let kite_uvs: [[f32; 2]; 4] = [[0.5, 1.0], [0.0, 0.0], [0.5, 0.0], [1.0, 0.0]];
    for (i, v) in [v0, v1, v2, v3].iter().enumerate() {
        positions.extend_from_slice(v);
        normals.extend_from_slice(&normal);
        uvs.extend_from_slice(&kite_uvs[i]);
        face_ids.push(face_id);
    }
    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

/// Build a pentagonal face (3 triangles fan) and append its vertices.
#[allow(clippy::too_many_arguments)]
fn add_pent_face(
    positions: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    face_ids: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    center: [f32; 3],
    corners: [[f32; 3]; 5],
    normal: [f32; 3],
    face_id: f32,
) {
    let base = (positions.len() / 3) as u32;
    // Center vertex
    positions.extend_from_slice(&center);
    normals.extend_from_slice(&normal);
    uvs.extend_from_slice(&[0.5, 0.5]);
    face_ids.push(face_id);
    // 5 corner vertices
    let corner_uvs: [[f32; 2]; 5] = [
        [0.5, 0.0],
        [0.95, 0.35],
        [0.78, 0.9],
        [0.22, 0.9],
        [0.05, 0.35],
    ];
    for (i, c) in corners.iter().enumerate() {
        positions.extend_from_slice(c);
        normals.extend_from_slice(&normal);
        uvs.extend_from_slice(&corner_uvs[i]);
        face_ids.push(face_id);
    }
    // Fan: center, i, i+1 for each of 5 corners
    for i in 0..5 {
        indices.extend_from_slice(&[base, base + 1 + i as u32, base + 1 + ((i + 1) % 5) as u32]);
    }
}
