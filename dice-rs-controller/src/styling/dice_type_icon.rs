use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use dice_rs::model::dice::DiceType;
use gtk4::prelude::*;
use smearor_commons_color::Color;
use smearor_commons_drawing::cairo::path::polyline_path;
use smearor_commons_drawing::pixel_buffer_from_surface;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use std::f64::consts::PI;

/// Icon size in pixels (square).
const ICON_SIZE: i32 = 80;

/// A screen-space point used by the icon drawings.
fn point(x: f64, y: f64) -> Vec2<Pixels> {
    Vec2::new(x, y)
}

/// Fill the polygon spanned by `points` in `color`.
fn fill_face(cr: &Context, points: &[Vec2<Pixels>], color: Color<f64>) {
    color.apply(cr);
    polyline_path(cr, points);
    cr.close_path();
    let _ = cr.fill();
}

/// Stroke a straight edge between `a` and `b`.
fn stroke_edge(cr: &Context, a: Vec2<Pixels>, b: Vec2<Pixels>) {
    polyline_path(cr, &[a, b]);
    let _ = cr.stroke();
}

/// An opaque RGB color for the icon drawings.
fn color(r: f64, g: f64, b: f64) -> Color<f64> {
    Color::<f64>::new(r, g, b)
}

/// A bluish-gray shading color: `lightness` on all channels, blue slightly lifted.
fn shade(lightness: f64) -> Color<f64> {
    color(lightness, lightness, lightness + 0.05)
}

/// Draw an isometric representation of the given dice type onto a Cairo context.
fn draw_dice(cr: &Context, dice_type: DiceType, size: f64) {
    let cx = size / 2.0;
    let cy = size / 2.0;
    let scale = size / 100.0;

    match dice_type {
        DiceType::D6 => draw_cube(cr, cx, cy, scale),
        DiceType::D20 => draw_icosahedron(cr, cx, cy, scale),
        DiceType::D10 => draw_pentagonal_trapezohedron(cr, cx, cy, scale, false),
        DiceType::D10X => draw_pentagonal_trapezohedron(cr, cx, cy, scale, true),
        DiceType::D4 => draw_tetrahedron(cr, cx, cy, scale),
        DiceType::D8 => draw_octahedron(cr, cx, cy, scale),
        DiceType::D12 => draw_dodecahedron(cr, cx, cy, scale),
    }
}

/// Draw a cube (D6) in isometric projection.
fn draw_cube(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 22.0 * scale;
    let cos30 = 30.0_f64.to_radians().cos();
    let sin30 = 30.0_f64.to_radians().sin();

    // Isometric projection: (x, y, z) -> screen
    let proj = |x: f64, y: f64, z: f64| point(cx + (x - y) * cos30 * s, cy + (x + y) * sin30 * s - z * s);

    // 8 cube vertices
    let v = [
        proj(-1.0, -1.0, -1.0),
        proj(1.0, -1.0, -1.0),
        proj(1.0, 1.0, -1.0),
        proj(-1.0, 1.0, -1.0),
        proj(-1.0, -1.0, 1.0),
        proj(1.0, -1.0, 1.0),
        proj(1.0, 1.0, 1.0),
        proj(-1.0, 1.0, 1.0),
    ];

    // Top face (lightest)
    fill_face(cr, &[v[4], v[5], v[6], v[7]], shade(0.55));

    // Right face (medium)
    fill_face(cr, &[v[5], v[6], v[2], v[1]], shade(0.4));

    // Left face (darkest)
    fill_face(cr, &[v[7], v[6], v[2], v[3]], color(0.28, 0.28, 0.32));

    // Edges
    color(0.15, 0.15, 0.18).apply(cr);
    cr.set_line_width(1.5);
    let edges = [
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // top
        (5, 1),
        (6, 2),
        (7, 3), // verticals
        (1, 2),
        (2, 3), // bottom visible
    ];
    for &(a, b) in &edges {
        stroke_edge(cr, v[a], v[b]);
    }
}

/// Draw a tetrahedron (D4).
fn draw_tetrahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 35.0 * scale;
    let h = s * 0.866; // sqrt(3)/2

    // 4 vertices: apex + 3 base corners
    let apex = point(cx, cy - h * 0.7);
    let v0 = point(cx - s * 0.866, cy + h * 0.4);
    let v1 = point(cx + s * 0.866, cy + h * 0.4);
    let v2 = point(cx, cy + h * 0.15);

    // Left face
    fill_face(cr, &[apex, v0, v2], shade(0.5));

    // Right face
    fill_face(cr, &[apex, v1, v2], shade(0.35));

    // Bottom face
    fill_face(cr, &[v0, v1, v2], shade(0.25));

    // Edges
    color(0.15, 0.15, 0.18).apply(cr);
    cr.set_line_width(1.5);
    let edges = [(apex, v0), (apex, v1), (v0, v2), (v1, v2), (v0, v1)];
    for &(a, b) in &edges {
        stroke_edge(cr, a, b);
    }
}

/// Draw an octahedron (D8).
fn draw_octahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 35.0 * scale;

    // 6 vertices: top, bottom, 4 equatorial
    let top = point(cx, cy - s);
    let bottom = point(cx, cy + s);
    let eq = [
        point(cx + s * 0.7, cy - s * 0.2),
        point(cx + s * 0.7, cy + s * 0.2),
        point(cx - s * 0.7, cy + s * 0.2),
        point(cx - s * 0.7, cy - s * 0.2),
    ];

    // Upper faces
    fill_face(cr, &[top, eq[0], eq[1]], shade(0.5));
    fill_face(cr, &[top, eq[3], eq[2]], shade(0.38));

    // Lower faces
    fill_face(cr, &[bottom, eq[1], eq[0]], shade(0.3));
    fill_face(cr, &[bottom, eq[2], eq[3]], shade(0.22));

    // Edges
    color(0.15, 0.15, 0.18).apply(cr);
    cr.set_line_width(1.5);
    let edges = [
        (top, eq[0]),
        (top, eq[1]),
        (top, eq[2]),
        (top, eq[3]),
        (bottom, eq[0]),
        (bottom, eq[1]),
        (bottom, eq[2]),
        (bottom, eq[3]),
        (eq[0], eq[1]),
        (eq[2], eq[3]),
    ];
    for &(a, b) in &edges {
        stroke_edge(cr, a, b);
    }
}

/// Draw an icosahedron (D20) — simplified as a triangular silhouette.
fn draw_icosahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 33.0 * scale;

    // Simplified icosahedron: top vertex, two rings of 5, bottom vertex
    let top = point(cx, cy - s);
    let bottom = point(cx, cy + s);

    let mut upper = Vec::with_capacity(5);
    let mut lower = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0;
        upper.push(point(cx + angle.cos() * s * 0.7, cy - s * 0.15 + angle.sin() * s * 0.3));
    }
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        lower.push(point(cx + angle.cos() * s * 0.7, cy + s * 0.15 + angle.sin() * s * 0.3));
    }

    // Upper triangles (top to upper ring)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.48 } else { 0.38 };
        fill_face(cr, &[top, upper[i], upper[j]], shade(lightness));
    }

    // Middle band (upper to lower)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.32 } else { 0.28 };
        fill_face(cr, &[upper[i], lower[i], lower[j]], shade(lightness));

        let lightness2 = if i % 2 == 0 { 0.35 } else { 0.3 };
        fill_face(cr, &[upper[i], upper[j], lower[j]], shade(lightness2));
    }

    // Lower triangles (lower ring to bottom)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.22 } else { 0.18 };
        fill_face(cr, &[bottom, lower[j], lower[i]], shade(lightness));
    }

    // Edges
    color(0.12, 0.12, 0.15).apply(cr);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        // top
        stroke_edge(cr, top, upper[i]);
        // upper ring
        stroke_edge(cr, upper[i], upper[j]);
        // middle
        stroke_edge(cr, upper[i], lower[i]);
        // lower ring
        stroke_edge(cr, lower[i], lower[j]);
        // bottom
        stroke_edge(cr, bottom, lower[i]);
    }
}

/// Draw a pentagonal trapezohedron (D10 / D10X).
fn draw_pentagonal_trapezohedron(cr: &Context, cx: f64, cy: f64, scale: f64, is_d10x: bool) {
    let s = 33.0 * scale;

    let top = point(cx, cy - s * 0.8);
    let bottom = point(cx, cy + s * 0.8);

    let mut upper = Vec::with_capacity(5);
    let mut lower = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0;
        upper.push(point(cx + angle.cos() * s * 0.6, cy - s * 0.1 + angle.sin() * s * 0.25));
    }
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        lower.push(point(cx + angle.cos() * s * 0.6, cy + s * 0.1 + angle.sin() * s * 0.25));
    }

    // Upper faces
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.48 } else { 0.38 };
        fill_face(cr, &[top, upper[i], upper[j]], shade(lightness));
    }

    // Lower faces
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.25 } else { 0.2 };
        fill_face(cr, &[bottom, lower[j], lower[i]], shade(lightness));
    }

    // Middle edges
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = 0.33;
        fill_face(cr, &[upper[i], lower[j], upper[j]], shade(lightness));
    }

    // Edges
    color(0.12, 0.12, 0.15).apply(cr);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        stroke_edge(cr, top, upper[i]);
        stroke_edge(cr, upper[i], upper[j]);
        stroke_edge(cr, upper[i], lower[j]);
        stroke_edge(cr, lower[i], lower[j]);
        stroke_edge(cr, bottom, lower[i]);
    }

    // Label for D10X
    if is_d10x {
        color(0.9, 0.9, 0.9).apply(cr);
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(10.0 * scale);
        cr.move_to(cx - 12.0 * scale, cy + 4.0 * scale);
        let _ = cr.show_text("00");
    }
}

/// Draw a dodecahedron (D12) — simplified as a pentagonal silhouette.
fn draw_dodecahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 32.0 * scale;

    // Outer pentagon
    let mut outer = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0;
        outer.push(point(cx + angle.cos() * s, cy + angle.sin() * s));
    }

    // Inner pentagon (smaller, rotated)
    let mut inner = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        inner.push(point(cx + angle.cos() * s * 0.45, cy + angle.sin() * s * 0.45));
    }

    // Fill outer faces (5 trapezoids)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let lightness = if i % 2 == 0 { 0.42 } else { 0.35 };
        fill_face(cr, &[outer[i], outer[j], inner[j], inner[i]], shade(lightness));
    }

    // Inner pentagon (top face)
    fill_face(cr, &inner, shade(0.52));

    // Edges
    color(0.12, 0.12, 0.15).apply(cr);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        // outer
        stroke_edge(cr, outer[i], outer[j]);
        // spokes
        stroke_edge(cr, outer[i], inner[i]);
        // inner
        stroke_edge(cr, inner[i], inner[j]);
    }
}

/// Render a dice type icon to a `gtk4::Image` widget.
pub fn create_icon(dice_type: DiceType) -> gtk4::Image {
    let mut surface = ImageSurface::create(Format::ARgb32, ICON_SIZE, ICON_SIZE).expect("failed to create Cairo surface");
    let cr = Context::new(&surface).expect("failed to create Cairo context");

    // Transparent background
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    cr.paint().expect("failed to paint background");

    draw_dice(&cr, dice_type, ICON_SIZE as f64);

    // Drop the context before accessing surface data (it holds a non-exclusive borrow).
    drop(cr);

    // ARgb32 pixels upload byte-exact as premultiplied BGRA.
    let buffer = pixel_buffer_from_surface(&mut surface).expect("failed to read surface pixels");
    let texture = buffer.to_memory_texture();

    let image = gtk4::Image::from_paintable(Some(&texture));
    let pixel_size = match dice_type {
        DiceType::D6 => 56,
        DiceType::D4 | DiceType::D10 | DiceType::D10X => 88,
        _ => 80,
    };
    image.set_pixel_size(pixel_size);
    image.add_css_class("dice-type-icon");
    image
}
