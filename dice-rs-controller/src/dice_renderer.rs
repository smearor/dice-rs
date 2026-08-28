use std::rc::Rc;

use glam::Mat4;
use glam::Quat;
use glam::Vec3;
use glow::HasContext;
use miette::Diagnostic;
use thiserror::Error;

use crate::models::DiceModel;

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

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/fragment.glsl");

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
