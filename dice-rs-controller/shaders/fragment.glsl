#version 300 es
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
