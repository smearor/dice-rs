use std::f64::consts::PI;

use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use dice_rs::model::dice::DiceType;
use gtk4::gdk::MemoryFormat;
use gtk4::gdk::MemoryTexture;
use gtk4::glib::Bytes;
use gtk4::prelude::*;

/// Icon size in pixels (square).
const ICON_SIZE: i32 = 80;

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
    let proj = |x: f64, y: f64, z: f64| -> (f64, f64) { (cx + (x - y) * cos30 * s, cy + (x + y) * sin30 * s - z * s) };

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
    cr.set_source_rgb(0.55, 0.55, 0.6);
    cr.new_path();
    cr.move_to(v[4].0, v[4].1);
    cr.line_to(v[5].0, v[5].1);
    cr.line_to(v[6].0, v[6].1);
    cr.line_to(v[7].0, v[7].1);
    cr.close_path();
    let _ = cr.fill();

    // Right face (medium)
    cr.set_source_rgb(0.4, 0.4, 0.45);
    cr.new_path();
    cr.move_to(v[5].0, v[5].1);
    cr.line_to(v[6].0, v[6].1);
    cr.line_to(v[2].0, v[2].1);
    cr.line_to(v[1].0, v[1].1);
    cr.close_path();
    let _ = cr.fill();

    // Left face (darkest)
    cr.set_source_rgb(0.28, 0.28, 0.32);
    cr.new_path();
    cr.move_to(v[7].0, v[7].1);
    cr.line_to(v[6].0, v[6].1);
    cr.line_to(v[2].0, v[2].1);
    cr.line_to(v[3].0, v[3].1);
    cr.close_path();
    let _ = cr.fill();

    // Edges
    cr.set_source_rgb(0.15, 0.15, 0.18);
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
        cr.new_path();
        cr.move_to(v[a].0, v[a].1);
        cr.line_to(v[b].0, v[b].1);
        let _ = cr.stroke();
    }
}

/// Draw a tetrahedron (D4).
fn draw_tetrahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 35.0 * scale;
    let h = s * 0.866; // sqrt(3)/2

    // 4 vertices: apex + 3 base corners
    let apex = (cx, cy - h * 0.7);
    let v0 = (cx - s * 0.866, cy + h * 0.4);
    let v1 = (cx + s * 0.866, cy + h * 0.4);
    let v2 = (cx, cy + h * 0.15);

    // Left face
    cr.set_source_rgb(0.5, 0.5, 0.55);
    cr.new_path();
    cr.move_to(apex.0, apex.1);
    cr.line_to(v0.0, v0.1);
    cr.line_to(v2.0, v2.1);
    cr.close_path();
    let _ = cr.fill();

    // Right face
    cr.set_source_rgb(0.35, 0.35, 0.4);
    cr.new_path();
    cr.move_to(apex.0, apex.1);
    cr.line_to(v1.0, v1.1);
    cr.line_to(v2.0, v2.1);
    cr.close_path();
    let _ = cr.fill();

    // Bottom face
    cr.set_source_rgb(0.25, 0.25, 0.3);
    cr.new_path();
    cr.move_to(v0.0, v0.1);
    cr.line_to(v1.0, v1.1);
    cr.line_to(v2.0, v2.1);
    cr.close_path();
    let _ = cr.fill();

    // Edges
    cr.set_source_rgb(0.15, 0.15, 0.18);
    cr.set_line_width(1.5);
    let edges = [(apex, v0), (apex, v1), (v0, v2), (v1, v2), (v0, v1)];
    for &(a, b) in &edges {
        cr.new_path();
        cr.move_to(a.0, a.1);
        cr.line_to(b.0, b.1);
        let _ = cr.stroke();
    }
}

/// Draw an octahedron (D8).
fn draw_octahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 35.0 * scale;

    // 6 vertices: top, bottom, 4 equatorial
    let top = (cx, cy - s);
    let bottom = (cx, cy + s);
    let eq = [
        (cx + s * 0.7, cy - s * 0.2),
        (cx + s * 0.7, cy + s * 0.2),
        (cx - s * 0.7, cy + s * 0.2),
        (cx - s * 0.7, cy - s * 0.2),
    ];

    // Upper faces
    cr.set_source_rgb(0.5, 0.5, 0.55);
    cr.new_path();
    cr.move_to(top.0, top.1);
    cr.line_to(eq[0].0, eq[0].1);
    cr.line_to(eq[1].0, eq[1].1);
    cr.close_path();
    let _ = cr.fill();

    cr.set_source_rgb(0.38, 0.38, 0.43);
    cr.new_path();
    cr.move_to(top.0, top.1);
    cr.line_to(eq[3].0, eq[3].1);
    cr.line_to(eq[2].0, eq[2].1);
    cr.close_path();
    let _ = cr.fill();

    // Lower faces
    cr.set_source_rgb(0.3, 0.3, 0.35);
    cr.new_path();
    cr.move_to(bottom.0, bottom.1);
    cr.line_to(eq[1].0, eq[1].1);
    cr.line_to(eq[0].0, eq[0].1);
    cr.close_path();
    let _ = cr.fill();

    cr.set_source_rgb(0.22, 0.22, 0.27);
    cr.new_path();
    cr.move_to(bottom.0, bottom.1);
    cr.line_to(eq[2].0, eq[2].1);
    cr.line_to(eq[3].0, eq[3].1);
    cr.close_path();
    let _ = cr.fill();

    // Edges
    cr.set_source_rgb(0.15, 0.15, 0.18);
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
        cr.new_path();
        cr.move_to(a.0, a.1);
        cr.line_to(b.0, b.1);
        let _ = cr.stroke();
    }
}

/// Draw an icosahedron (D20) — simplified as a triangular silhouette.
fn draw_icosahedron(cr: &Context, cx: f64, cy: f64, scale: f64) {
    let s = 33.0 * scale;

    // Simplified icosahedron: top vertex, two rings of 5, bottom vertex
    let top = (cx, cy - s);
    let bottom = (cx, cy + s);

    let mut upper = Vec::with_capacity(5);
    let mut lower = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0;
        upper.push((cx + angle.cos() * s * 0.7, cy - s * 0.15 + angle.sin() * s * 0.3));
    }
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        lower.push((cx + angle.cos() * s * 0.7, cy + s * 0.15 + angle.sin() * s * 0.3));
    }

    // Upper triangles (top to upper ring)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.48 } else { 0.38 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(top.0, top.1);
        cr.line_to(upper[i].0, upper[i].1);
        cr.line_to(upper[j].0, upper[j].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Middle band (upper to lower)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.32 } else { 0.28 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(lower[i].0, lower[i].1);
        cr.line_to(lower[j].0, lower[j].1);
        cr.close_path();
        let _ = cr.fill();

        let shade2 = if i % 2 == 0 { 0.35 } else { 0.3 };
        cr.set_source_rgb(shade2, shade2, shade2 + 0.05);
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(upper[j].0, upper[j].1);
        cr.line_to(lower[j].0, lower[j].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Lower triangles (lower ring to bottom)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.22 } else { 0.18 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(bottom.0, bottom.1);
        cr.line_to(lower[j].0, lower[j].1);
        cr.line_to(lower[i].0, lower[i].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Edges
    cr.set_source_rgb(0.12, 0.12, 0.15);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        // top
        cr.new_path();
        cr.move_to(top.0, top.1);
        cr.line_to(upper[i].0, upper[i].1);
        let _ = cr.stroke();
        // upper ring
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(upper[j].0, upper[j].1);
        let _ = cr.stroke();
        // middle
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(lower[i].0, lower[i].1);
        let _ = cr.stroke();
        // lower ring
        cr.new_path();
        cr.move_to(lower[i].0, lower[i].1);
        cr.line_to(lower[j].0, lower[j].1);
        let _ = cr.stroke();
        // bottom
        cr.new_path();
        cr.move_to(bottom.0, bottom.1);
        cr.line_to(lower[i].0, lower[i].1);
        let _ = cr.stroke();
    }
}

/// Draw a pentagonal trapezohedron (D10 / D10X).
fn draw_pentagonal_trapezohedron(cr: &Context, cx: f64, cy: f64, scale: f64, is_d10x: bool) {
    let s = 33.0 * scale;

    let top = (cx, cy - s * 0.8);
    let bottom = (cx, cy + s * 0.8);

    let mut upper = Vec::with_capacity(5);
    let mut lower = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0;
        upper.push((cx + angle.cos() * s * 0.6, cy - s * 0.1 + angle.sin() * s * 0.25));
    }
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        lower.push((cx + angle.cos() * s * 0.6, cy + s * 0.1 + angle.sin() * s * 0.25));
    }

    // Upper faces
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.48 } else { 0.38 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(top.0, top.1);
        cr.line_to(upper[i].0, upper[i].1);
        cr.line_to(upper[j].0, upper[j].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Lower faces
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.25 } else { 0.2 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(bottom.0, bottom.1);
        cr.line_to(lower[j].0, lower[j].1);
        cr.line_to(lower[i].0, lower[i].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Middle edges
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = 0.33;
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(lower[j].0, lower[j].1);
        cr.line_to(upper[j].0, upper[j].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Edges
    cr.set_source_rgb(0.12, 0.12, 0.15);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        cr.new_path();
        cr.move_to(top.0, top.1);
        cr.line_to(upper[i].0, upper[i].1);
        let _ = cr.stroke();
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(upper[j].0, upper[j].1);
        let _ = cr.stroke();
        cr.new_path();
        cr.move_to(upper[i].0, upper[i].1);
        cr.line_to(lower[j].0, lower[j].1);
        let _ = cr.stroke();
        cr.new_path();
        cr.move_to(lower[i].0, lower[i].1);
        cr.line_to(lower[j].0, lower[j].1);
        let _ = cr.stroke();
        cr.new_path();
        cr.move_to(bottom.0, bottom.1);
        cr.line_to(lower[i].0, lower[i].1);
        let _ = cr.stroke();
    }

    // Label for D10X
    if is_d10x {
        cr.set_source_rgb(0.9, 0.9, 0.9);
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
        outer.push((cx + angle.cos() * s, cy + angle.sin() * s));
    }

    // Inner pentagon (smaller, rotated)
    let mut inner = Vec::with_capacity(5);
    for i in 0..5 {
        let angle = (i as f64) * 2.0 * PI / 5.0 - PI / 2.0 + PI / 5.0;
        inner.push((cx + angle.cos() * s * 0.45, cy + angle.sin() * s * 0.45));
    }

    // Fill outer faces (5 trapezoids)
    for i in 0..5 {
        let j = (i + 1) % 5;
        let shade = if i % 2 == 0 { 0.42 } else { 0.35 };
        cr.set_source_rgb(shade, shade, shade + 0.05);
        cr.new_path();
        cr.move_to(outer[i].0, outer[i].1);
        cr.line_to(outer[j].0, outer[j].1);
        cr.line_to(inner[j].0, inner[j].1);
        cr.line_to(inner[i].0, inner[i].1);
        cr.close_path();
        let _ = cr.fill();
    }

    // Inner pentagon (top face)
    cr.set_source_rgb(0.52, 0.52, 0.57);
    cr.new_path();
    for (i, point) in inner.iter().enumerate().take(5) {
        if i == 0 {
            cr.move_to(point.0, point.1);
        } else {
            cr.line_to(point.0, point.1);
        }
    }
    cr.close_path();
    let _ = cr.fill();

    // Edges
    cr.set_source_rgb(0.12, 0.12, 0.15);
    cr.set_line_width(1.0);
    for i in 0..5 {
        let j = (i + 1) % 5;
        // outer
        cr.new_path();
        cr.move_to(outer[i].0, outer[i].1);
        cr.line_to(outer[j].0, outer[j].1);
        let _ = cr.stroke();
        // spokes
        cr.new_path();
        cr.move_to(outer[i].0, outer[i].1);
        cr.line_to(inner[i].0, inner[i].1);
        let _ = cr.stroke();
        // inner
        cr.new_path();
        cr.move_to(inner[i].0, inner[i].1);
        cr.line_to(inner[j].0, inner[j].1);
        let _ = cr.stroke();
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

    // Convert surface to pixel data
    let data = surface.data().expect("failed to get surface data");
    let bytes = Bytes::from(&data.to_vec());
    let rowstride = (ICON_SIZE * 4) as usize;
    let texture = MemoryTexture::new(ICON_SIZE, ICON_SIZE, MemoryFormat::B8g8r8a8, &bytes, rowstride);

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
