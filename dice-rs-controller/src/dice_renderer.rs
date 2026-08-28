use std::rc::Rc;

use glam::Mat4;
use glam::Quat;
use glam::Vec3;
use glow::HasContext;
use miette::Diagnostic;
use thiserror::Error;

use crate::dice_model::DiceModel;

/// Errors that can occur during OpenGL renderer initialization or rendering.
#[derive(Debug, Error, Diagnostic)]
pub enum RendererError {
    #[error("failed to create OpenGL vertex array object")]
    #[diagnostic(code(dice_renderer::vao_creation_failed))]
    VaoCreationFailed,

    #[error("failed to create OpenGL vertex buffer object")]
    #[diagnostic(code(dice_renderer::vbo_creation_failed))]
    VboCreationFailed,

    #[error("failed to create OpenGL element buffer object")]
    #[diagnostic(code(dice_renderer::ebo_creation_failed))]
    EboCreationFailed,

    #[error("failed to create OpenGL {shader_type} shader")]
    #[diagnostic(code(dice_renderer::shader_creation_failed))]
    ShaderCreationFailed { shader_type: String },

    #[error("{shader_type} shader compilation failed: {log}")]
    #[diagnostic(code(dice_renderer::shader_compilation_failed))]
    ShaderCompilationFailed { shader_type: String, log: String },

    #[error("failed to create OpenGL shader program")]
    #[diagnostic(code(dice_renderer::program_creation_failed))]
    ProgramCreationFailed,

    #[error("shader program linking failed: {log}")]
    #[diagnostic(code(dice_renderer::program_linking_failed))]
    ProgramLinkingFailed { log: String },

    #[error("shader attribute '{name}' not found in program")]
    #[diagnostic(code(dice_renderer::attribute_not_found))]
    AttributeNotFound { name: String },

    #[error("shader uniform '{name}' not found in program")]
    #[diagnostic(code(dice_renderer::uniform_not_found))]
    UniformNotFound { name: String },
}

type Result<T> = std::result::Result<T, RendererError>;

/// OpenGL renderer for 3D dice models using glow + glam.
pub struct DiceRenderer {
    pub gl: Rc<glow::Context>,
    program: <glow::Context as HasContext>::Program,
    vao: <glow::Context as HasContext>::VertexArray,
    vbo: <glow::Context as HasContext>::Buffer,
    ebo: <glow::Context as HasContext>::Buffer,
    index_count: i32,
    /// Light direction for diffuse lighting (normalized).
    light_dir: Vec3,
    /// 0=tri/kite, 1=quad, 2=pentagon — controls shader edge detection.
    face_shape: i32,
    /// 1 if D10X (face values shown as 00, 10, ..., 90).
    is_d10x: i32,
}

const VERTEX_SHADER_SOURCE: &str = r#"#version 300 es
precision highp float;
in vec3 a_pos;
in vec3 a_normal;
in vec2 a_uv;
in float a_face_id;
uniform mat4 u_mvp;
uniform mat4 u_model;
out vec3 v_normal;
out vec3 v_frag_pos;
out vec2 v_uv;
out float v_face_id;
void main() {
    v_normal = mat3(u_model) * a_normal;
    v_frag_pos = vec3(u_model * vec4(a_pos, 1.0));
    v_uv = a_uv;
    v_face_id = a_face_id;
    gl_Position = u_mvp * vec4(a_pos, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"#version 300 es
precision highp float;
in vec3 v_normal;
in vec3 v_frag_pos;
in vec2 v_uv;
in float v_face_id;
uniform vec3 u_light_dir;
uniform vec3 u_base_color;
uniform vec3 u_edge_color;
uniform int u_face_shape; // 0=tri/kite, 1=quad, 2=pentagon
uniform int u_is_d10x;    // 1 if D10X (face values x10)
out vec4 frag_color;

const float PIP_RADIUS = 0.07;
const vec3 INK_COLOR = vec3(0.02, 0.02, 0.02);
const float EDGE_WIDTH = 0.05;

// --- Edge detection ---

float tri_edge_dist(vec2 uv) {
    // Triangle UVs: [0,0], [1,0], [0.5,1]
    float d_bottom = uv.y;
    float d_left = (2.0 * uv.x - uv.y) / 2.236068;
    float d_right = (2.0 - 2.0 * uv.x - uv.y) / 2.236068;
    return min(d_bottom, min(d_left, d_right));
}

float kite_edge_dist(vec2 uv) {
    // Kite UVs: apex [0.5,1.0], left [0.0,0.0], center [0.5,0.0], right [1.0,0.0]
    // Outer edges: left (x-0.5y=0), bottom (y=0), right (x+0.5y=1)
    float d_left = (uv.x - 0.5 * uv.y) / 1.118034;
    float d_bottom = uv.y;
    float d_right = (1.0 - uv.x - 0.5 * uv.y) / 1.118034;
    return min(d_left, min(d_bottom, d_right));
}

float quad_edge_dist(vec2 uv) {
    return min(min(uv.x, uv.y), min(1.0 - uv.x, 1.0 - uv.y));
}

float pent_edge_dist(vec2 uv) {
    // Pentagon UV corners: [0.5,0], [0.95,0.35], [0.78,0.9], [0.22,0.9], [0.05,0.35]
    float d0 = (0.45 * uv.y - 0.35 * (uv.x - 0.5)) / 0.5701;
    float d1 = (-0.17 * (uv.y - 0.35) - 0.55 * (uv.x - 0.95)) / 0.5757;
    float d2 = 0.9 - uv.y;
    float d3 = (-0.17 * (uv.y - 0.9) + 0.55 * (uv.x - 0.22)) / 0.5757;
    float d4 = (0.45 * (uv.y - 0.35) + 0.35 * (uv.x - 0.05)) / 0.5701;
    return min(d0, min(d1, min(d2, min(d3, d4))));
}

float compute_edge_dist() {
    if (u_face_shape == 1) return quad_edge_dist(v_uv);
    if (u_face_shape == 2) return pent_edge_dist(v_uv);
    if (u_face_shape == 3) return kite_edge_dist(v_uv);
    return tri_edge_dist(v_uv);
}

// --- Pip rendering (D6 only) ---

bool in_pip(vec2 uv, vec2 center) {
    return distance(uv, center) < PIP_RADIUS;
}

bool is_pip_fragment(float face_id, vec2 uv) {
    int face = int(face_id + 0.5);
    if (face == 1) return in_pip(uv, vec2(0.5, 0.5));
    if (face == 2) return in_pip(uv, vec2(0.25, 0.25)) || in_pip(uv, vec2(0.75, 0.75));
    if (face == 3) return in_pip(uv, vec2(0.25, 0.25)) || in_pip(uv, vec2(0.5, 0.5)) || in_pip(uv, vec2(0.75, 0.75));
    if (face == 4) return in_pip(uv, vec2(0.25, 0.25)) || in_pip(uv, vec2(0.75, 0.25)) || in_pip(uv, vec2(0.25, 0.75)) || in_pip(uv, vec2(0.75, 0.75));
    if (face == 5) return in_pip(uv, vec2(0.25, 0.25)) || in_pip(uv, vec2(0.75, 0.25)) || in_pip(uv, vec2(0.5, 0.5)) || in_pip(uv, vec2(0.25, 0.75)) || in_pip(uv, vec2(0.75, 0.75));
    if (face == 6) return in_pip(uv, vec2(0.25, 0.2)) || in_pip(uv, vec2(0.75, 0.2)) || in_pip(uv, vec2(0.25, 0.5)) || in_pip(uv, vec2(0.75, 0.5)) || in_pip(uv, vec2(0.25, 0.8)) || in_pip(uv, vec2(0.75, 0.8));
    return false;
}

// --- 7-segment number rendering ---

int digit_pattern(int d) {
    if (d == 0) return 63;
    if (d == 1) return 6;
    if (d == 2) return 91;
    if (d == 3) return 79;
    if (d == 4) return 102;
    if (d == 5) return 109;
    if (d == 6) return 125;
    if (d == 7) return 7;
    if (d == 8) return 127;
    if (d == 9) return 111;
    return 0;
}

bool in_rect(vec2 p, vec2 c, float hw, float hh) {
    vec2 d = abs(p - c);
    return d.x < hw && d.y < hh;
}

bool in_segment(vec2 p, int seg) {
    float hw = 0.35;
    float hh = 0.5;
    float sw = 0.12;
    if (seg == 0) return in_rect(p, vec2(0.0, hh), hw, sw * 0.5);
    if (seg == 1) return in_rect(p, vec2(hw, hh * 0.5), sw * 0.5, hh * 0.5);
    if (seg == 2) return in_rect(p, vec2(hw, -hh * 0.5), sw * 0.5, hh * 0.5);
    if (seg == 3) return in_rect(p, vec2(0.0, -hh), hw, sw * 0.5);
    if (seg == 4) return in_rect(p, vec2(-hw, -hh * 0.5), sw * 0.5, hh * 0.5);
    if (seg == 5) return in_rect(p, vec2(-hw, hh * 0.5), sw * 0.5, hh * 0.5);
    if (seg == 6) return in_rect(p, vec2(0.0, 0.0), hw, sw * 0.5);
    return false;
}

bool in_underline(vec2 p, float hw) {
    return in_rect(p, vec2(0.0, -0.62), hw, 0.04);
}

float digit_x_offset(int d, float scale) {
    if (d == 1) return -0.35 * scale;
    return 0.0;
}

bool is_number_fragment(float face_id, vec2 uv) {
    int face = int(face_id + 0.5);
    if (face < 1) return false;
    int value = face;
    if (u_is_d10x == 1) {
        value = (face - 1) * 10;
    }
    int tens = value / 10;
    int ones = value % 10;
    float center_y = 0.5;
    if (u_face_shape == 0) center_y = 0.33;
    if (u_face_shape == 3) center_y = 0.35;
    if (tens > 0 || u_is_d10x == 1) {
        float scale = 0.15;
        vec2 center_tens = vec2(0.35 + digit_x_offset(tens, scale), center_y);
        vec2 center_ones = vec2(0.65 + digit_x_offset(ones, scale), center_y);
        int pat_t = digit_pattern(tens);
        int pat_o = digit_pattern(ones);
        for (int s = 0; s < 7; s++) {
            if ((pat_t & (1 << s)) != 0 && in_segment((uv - center_tens) / scale, s)) return true;
            if ((pat_o & (1 << s)) != 0 && in_segment((uv - center_ones) / scale, s)) return true;
        }
        if (tens == 6 && in_underline((uv - center_tens) / scale, 0.25)) return true;
        if (ones == 6 && in_underline((uv - center_ones) / scale, 0.25)) return true;
    } else {
        float scale = 0.35;
        vec2 center = vec2(0.5 + digit_x_offset(ones, scale), center_y);
        int pat = digit_pattern(ones);
        for (int s = 0; s < 7; s++) {
            if ((pat & (1 << s)) != 0 && in_segment((uv - center) / scale, s)) return true;
        }
        if (ones == 6 && in_underline((uv - center) / scale, 0.35)) return true;
    }
    return false;
}

void main() {
    vec3 normal = normalize(v_normal);
    float diff = max(dot(normal, normalize(u_light_dir)), 0.0);
    bool is_ink = false;
    if (u_face_shape == 1) {
        is_ink = is_pip_fragment(v_face_id, v_uv);
    } else {
        is_ink = is_number_fragment(v_face_id, v_uv);
    }
    float ed = compute_edge_dist();
    bool is_edge = ed < EDGE_WIDTH && !is_ink;
    vec3 base = u_base_color;
    if (is_ink) base = INK_COLOR;
    if (is_edge) base = u_edge_color;
    vec3 ambient = base * 0.35;
    vec3 diffuse = base * diff * 0.65;
    frag_color = vec4(ambient + diffuse, 1.0);
}
"#;

impl DiceRenderer {
    /// Create a new renderer with the given GL context.
    pub fn new(gl: Rc<glow::Context>, model: &DiceModel) -> Result<Self> {
        let program = unsafe { Self::compile_program(&gl)? };

        let vao = unsafe { gl.create_vertex_array().map_err(|_| RendererError::VaoCreationFailed)? };
        let vbo = unsafe { gl.create_buffer().map_err(|_| RendererError::VboCreationFailed)? };
        let ebo = unsafe { gl.create_buffer().map_err(|_| RendererError::EboCreationFailed)? };

        unsafe {
            gl.bind_vertex_array(Some(vao));

            // Interleaved vertex data: pos(3) + normal(3) + uv(2) + face_id(1) = 9 floats per vertex
            let stride = 9 * std::mem::size_of::<f32>() as i32;
            let vertex_count = model.positions.len() / 3;
            let mut vertex_data: Vec<f32> = Vec::with_capacity(vertex_count * 9);
            for i in 0..vertex_count {
                vertex_data.extend_from_slice(&model.positions[i * 3..i * 3 + 3]);
                vertex_data.extend_from_slice(&model.normals[i * 3..i * 3 + 3]);
                vertex_data.extend_from_slice(&model.uvs[i * 2..i * 2 + 2]);
                vertex_data.push(model.face_ids[i]);
            }

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_data.align_to::<u8>().1, glow::STATIC_DRAW);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            let index_data: &[u8] = bytemuck::cast_slice(&model.indices);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, index_data, glow::STATIC_DRAW);

            let pos_loc = gl
                .get_attrib_location(program, "a_pos")
                .ok_or(RendererError::AttributeNotFound { name: "a_pos".to_string() })?;
            gl.enable_vertex_attrib_array(pos_loc);
            gl.vertex_attrib_pointer_f32(pos_loc, 3, glow::FLOAT, false, stride, 0);

            let normal_loc = gl
                .get_attrib_location(program, "a_normal")
                .ok_or(RendererError::AttributeNotFound { name: "a_normal".to_string() })?;
            gl.enable_vertex_attrib_array(normal_loc);
            gl.vertex_attrib_pointer_f32(normal_loc, 3, glow::FLOAT, false, stride, 3 * std::mem::size_of::<f32>() as i32);

            let uv_loc = gl
                .get_attrib_location(program, "a_uv")
                .ok_or(RendererError::AttributeNotFound { name: "a_uv".to_string() })?;
            gl.enable_vertex_attrib_array(uv_loc);
            gl.vertex_attrib_pointer_f32(uv_loc, 2, glow::FLOAT, false, stride, 6 * std::mem::size_of::<f32>() as i32);

            let face_id_loc = gl
                .get_attrib_location(program, "a_face_id")
                .ok_or(RendererError::AttributeNotFound { name: "a_face_id".to_string() })?;
            gl.enable_vertex_attrib_array(face_id_loc);
            gl.vertex_attrib_pointer_f32(face_id_loc, 1, glow::FLOAT, false, stride, 8 * std::mem::size_of::<f32>() as i32);

            gl.bind_vertex_array(None);
        }

        Ok(Self {
            gl,
            program,
            vao,
            vbo,
            ebo,
            index_count: model.indices.len() as i32,
            light_dir: Vec3::new(0.5, -1.0, 0.3).normalize(),
            face_shape: model.face_shape,
            is_d10x: if model.is_d10x { 1 } else { 0 },
        })
    }

    /// Render the dice model with the given orientation and aspect ratio.
    #[allow(deprecated)]
    pub fn render(&self, orientation: Quat, aspect: f32, base_color: [f32; 3], edge_color: [f32; 3]) -> Result<()> {
        let gl = &self.gl;

        let model_matrix = Mat4::from_quat(orientation);
        let eye = Vec3::new(2.0, 2.0, 3.0);
        let target = Vec3::ZERO;
        let view_matrix = Mat4::look_at_rh(eye, target, Vec3::Y);
        let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
        let mvp = projection * view_matrix * model_matrix;

        unsafe {
            gl.use_program(Some(self.program));

            let mvp_loc = gl
                .get_uniform_location(self.program, "u_mvp")
                .ok_or(RendererError::UniformNotFound { name: "u_mvp".to_string() })?;
            gl.uniform_matrix_4_f32_slice(Some(&mvp_loc), false, &mvp.to_cols_array());

            let model_loc = gl
                .get_uniform_location(self.program, "u_model")
                .ok_or(RendererError::UniformNotFound { name: "u_model".to_string() })?;
            gl.uniform_matrix_4_f32_slice(Some(&model_loc), false, &model_matrix.to_cols_array());

            let light_loc = gl.get_uniform_location(self.program, "u_light_dir").ok_or(RendererError::UniformNotFound {
                name: "u_light_dir".to_string(),
            })?;
            gl.uniform_3_f32(Some(&light_loc), self.light_dir.x, self.light_dir.y, self.light_dir.z);

            let color_loc = gl.get_uniform_location(self.program, "u_base_color").ok_or(RendererError::UniformNotFound {
                name: "u_base_color".to_string(),
            })?;
            gl.uniform_3_f32(Some(&color_loc), base_color[0], base_color[1], base_color[2]);

            let edge_loc = gl.get_uniform_location(self.program, "u_edge_color").ok_or(RendererError::UniformNotFound {
                name: "u_edge_color".to_string(),
            })?;
            gl.uniform_3_f32(Some(&edge_loc), edge_color[0], edge_color[1], edge_color[2]);

            let shape_loc = gl.get_uniform_location(self.program, "u_face_shape").ok_or(RendererError::UniformNotFound {
                name: "u_face_shape".to_string(),
            })?;
            gl.uniform_1_i32(Some(&shape_loc), self.face_shape);

            let d10x_loc = gl.get_uniform_location(self.program, "u_is_d10x").ok_or(RendererError::UniformNotFound {
                name: "u_is_d10x".to_string(),
            })?;
            gl.uniform_1_i32(Some(&d10x_loc), self.is_d10x);

            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);

            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }

        Ok(())
    }

    unsafe fn compile_program(gl: &Rc<glow::Context>) -> Result<<glow::Context as HasContext>::Program> {
        unsafe {
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).map_err(|_| RendererError::ShaderCreationFailed {
                shader_type: "vertex".to_string(),
            })?;
            gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                let log = gl.get_shader_info_log(vertex_shader);
                return Err(RendererError::ShaderCompilationFailed {
                    shader_type: "vertex".to_string(),
                    log,
                });
            }

            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).map_err(|_| RendererError::ShaderCreationFailed {
                shader_type: "fragment".to_string(),
            })?;
            gl.shader_source(fragment_shader, FRAGMENT_SHADER_SOURCE);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                let log = gl.get_shader_info_log(fragment_shader);
                return Err(RendererError::ShaderCompilationFailed {
                    shader_type: "fragment".to_string(),
                    log,
                });
            }

            let program = gl.create_program().map_err(|_| RendererError::ProgramCreationFailed)?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                return Err(RendererError::ProgramLinkingFailed { log });
            }

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(program)
        }
    }
}

impl Drop for DiceRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_buffer(self.ebo);
            self.gl.delete_program(self.program);
        }
    }
}
