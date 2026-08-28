#version 300 es
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
