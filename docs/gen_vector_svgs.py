#!/usr/bin/env python3
"""Generate SVG visualizations of dice reference vector tables.

Each vector represents the gravity direction (pointing from the die center
toward the face that is touching the table) when a particular face is down.
The SVGs use three projection views (isometric, front, top) to disambiguate
overlapping vectors.
"""

import math
import textwrap
from pathlib import Path

# --- Vector tables (ported from dice-rs/src/service/interpreter/vectors.rs) ---

D6_VECTORS = [
    (-64, 0, 0),    # face 1
    (0, 0, 64),     # face 2
    (0, 64, 0),     # face 3
    (0, -64, 0),    # face 4
    (0, 0, -64),    # face 5
    (64, 0, 0),     # face 6
]

D20_VECTORS = [
    (-64, 0, -22),    # face 1
    (42, -42, 40),    # face 2
    (0, 22, -64),     # face 3
    (0, 22, 64),      # face 4
    (-42, -42, 42),   # face 5
    (22, 64, 0),      # face 6
    (-42, -42, -42),  # face 7
    (64, 0, -22),     # face 8
    (-22, 64, 0),     # face 9
    (42, -42, -42),   # face 10
    (-42, 42, 42),    # face 11
    (22, -64, 0),     # face 12
    (-64, 0, 22),     # face 13
    (42, 42, 42),     # face 14
    (-22, -64, 0),    # face 15
    (42, 42, -42),    # face 16
    (0, -22, -64),    # face 17
    (0, -22, 64),     # face 18
    (-42, 42, -42),   # face 19
    (64, 0, 22),      # face 20
]

D24_VECTORS = [
    (20, -60, -20),   # face 1
    (20, 0, 60),      # face 2
    (-40, -40, 40),   # face 3
    (-60, 0, 20),     # face 4
    (40, 20, 40),     # face 5
    (-20, -60, -20),  # face 6
    (20, 60, 20),     # face 7
    (-40, 20, -40),   # face 8
    (-40, 40, 40),    # face 9
    (-20, 0, 60),     # face 10
    (-20, -60, 20),   # face 11
    (60, 0, 20),      # face 12
    (-60, 0, -20),    # face 13
    (20, 60, -20),    # face 14
    (20, 0, -60),     # face 15
    (40, -20, -40),   # face 16
    (-20, 60, -20),   # face 17
    (-40, -40, -40),  # face 18
    (40, -20, 40),    # face 19
    (20, -60, 20),    # face 20
    (60, 0, -20),     # face 21
    (40, 20, -40),    # face 22
    (-20, 0, -60),    # face 23
    (-20, 60, 20),    # face 24
]

# --- Projections ---

COS30 = math.cos(math.radians(30))
SIN30 = math.sin(math.radians(30))

FACE_COLORS = [
    "#e74c3c", "#3498db", "#2ecc71", "#f39c12", "#9b59b6",
    "#1abc9c", "#e67e22", "#34495e", "#e91e63", "#00bcd4",
    "#8bc34a", "#ff5722", "#607d8b", "#795548", "#cddc39",
    "#ff9800", "#673ab7", "#4caf50", "#ffc107", "#03a9f4",
    "#880e4f", "#bf360c", "#0d47a1", "#1b5e20",
]


def project_iso(x, y, z, scale, ox, oy):
    sx = ox + (x - y) * COS30 * scale
    sy = oy + (x + y) * SIN30 * scale - z * scale
    return sx, sy


def project_front(x, y, z, scale, ox, oy):
    return ox + x * scale, oy - z * scale


def project_top(x, y, z, scale, ox, oy):
    return ox + x * scale, oy + y * scale


def shift_label(sx, sy, ox, oy):
    rx = sx - ox
    ry = sy - oy
    dist = math.hypot(rx, ry)
    if dist < 1:
        return sx + 10, sy + 14, "start"
    push = 12
    nx = rx / dist * push
    ny = ry / dist * push
    anchor = "start" if nx >= 0 else "end"
    return sx + nx, sy + ny + 4, anchor


def make_view(vectors, project_fn, title, scale, ox, oy):
    parts = []
    axis_len = 75
    axis_defs = [
        ("+X", (axis_len, 0, 0), "#e74c3c"),
        ("-X", (-axis_len, 0, 0), "#e74c3c"),
        ("+Y", (0, axis_len, 0), "#2ecc71"),
        ("-Y", (0, -axis_len, 0), "#2ecc71"),
        ("+Z", (0, 0, axis_len), "#3498db"),
        ("-Z", (0, 0, -axis_len), "#3498db"),
    ]
    for label, (ax, ay, az), color in axis_defs:
        ex, ey = project_fn(ax, ay, az, scale, ox, oy)
        parts.append(
            f'    <line x1="{ox:.0f}" y1="{oy:.0f}" '
            f'x2="{ex:.0f}" y2="{ey:.0f}" '
            f'stroke="{color}" stroke-width="1" '
            f'stroke-dasharray="3,2" opacity="0.4"/>\n'
        )
        parts.append(
            f'    <text x="{ex:.0f}" y="{ey:.0f}" '
            f'fill="{color}" font-size="9" '
            f'font-weight="bold" text-anchor="middle" '
            f'dy="-3" opacity="0.6">{label}</text>\n'
        )
    r = 64 * scale
    parts.append(
        f'    <circle cx="{ox:.0f}" cy="{oy:.0f}" r="{r:.0f}" '
        f'fill="none" stroke="#ddd" stroke-width="0.8" '
        f'stroke-dasharray="2,2"/>\n'
    )
    for i, (x, y, z) in enumerate(vectors):
        face = i + 1
        color = FACE_COLORS[i % len(FACE_COLORS)]
        sx, sy = project_fn(x, y, z, scale, ox, oy)
        lx, ly, anchor = shift_label(sx, sy, ox, oy)
        parts.append(
            f'    <line x1="{ox:.0f}" y1="{oy:.0f}" '
            f'x2="{sx:.0f}" y2="{sy:.0f}" '
            f'stroke="{color}" stroke-width="1.5" opacity="0.8"/>\n'
        )
        parts.append(
            f'    <circle cx="{sx:.0f}" cy="{sy:.0f}" r="3.5" '
            f'fill="{color}" stroke="#333" stroke-width="0.5"/>\n'
        )
        parts.append(
            f'    <text x="{lx:.0f}" y="{ly:.0f}" '
            f'fill="#333" font-size="10" font-weight="bold" '
            f'text-anchor="{anchor}">{face}</text>\n'
        )
    parts.append(
        f'    <circle cx="{ox:.0f}" cy="{oy:.0f}" r="2.5" fill="#333"/>\n'
    )
    parts.append(
        f'    <text x="{ox:.0f}" y="{oy + r + 22:.0f}" '
        f'text-anchor="middle" font-size="11" '
        f'font-weight="bold" fill="#555">{title}</text>\n'
    )
    return "".join(parts)


def make_svg(name, vectors, title):
    scale = 1.4
    view_size = 200
    margin = 30
    views = [
        ("Isometric (X-Y-Z)", project_iso),
        ("Front (X-Z)", project_front),
        ("Top (X-Y)", project_top),
    ]
    total_width = view_size * 3 + margin * 5
    height = view_size + margin * 3 + 50
    svg_parts = []
    svg_parts.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" '
        f'viewBox="0 0 {total_width:.0f} {height:.0f}" '
        f'font-family="sans-serif" font-size="11">\n'
    )
    svg_parts.append(
        f'  <rect x="0" y="0" width="{total_width:.0f}" '
        f'height="{height:.0f}" fill="#fafafa" rx="8"/>\n'
    )
    svg_parts.append(
        f'  <text x="{total_width / 2:.0f}" y="22" '
        f'text-anchor="middle" font-size="15" font-weight="bold" '
        f'fill="#333">{title}</text>\n'
    )
    for idx, (view_title, project_fn) in enumerate(views):
        cx = margin + (view_size + margin) * idx + view_size / 2
        cy = margin + 30 + view_size / 2
        svg_parts.append(
            f'  <rect x="{cx - view_size / 2:.0f}" y="{margin + 15:.0f}" '
            f'width="{view_size:.0f}" height="{view_size:.0f}" '
            f'fill="white" stroke="#eee" stroke-width="1" rx="4"/>\n'
        )
        svg_parts.append(make_view(vectors, project_fn, view_title, scale, cx, cy))
    legend_y = height - 12
    svg_parts.append(
        f'  <text x="10" y="{legend_y:.0f}" font-size="9" fill="#999">'
        f'Each colored arrow = gravity vector when that face is down. '
        f'Red=X, Green=Y, Blue=Z (dashed axes). '
        f'Circle = unit sphere (|v|=64).'
        f'</text>\n'
    )
    svg_parts.append("</svg>\n")
    return "".join(svg_parts)


def main():
    out_dir = Path(__file__).parent
    out_dir.mkdir(exist_ok=True)
    svgs = [
        ("d6_vectors", D6_VECTORS, "D6 Reference Vectors (6 faces)"),
        ("d20_vectors", D20_VECTORS, "D20 Reference Vectors (20 faces)"),
        ("d24_vectors", D24_VECTORS, "D24 Reference Vectors (24 faces)"),
    ]
    for name, vectors, title in svgs:
        svg = make_svg(name, vectors, title)
        path = out_dir / f"{name}.svg"
        path.write_text(svg, encoding="utf-8")
        print(f"  Generated {path}")
    html = textwrap.dedent("""\
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <title>Dice Reference Vector Diagrams</title>
          <style>
            body { font-family: sans-serif; max-width: 1100px; margin: 2em auto; padding: 0 1em; }
            h1 { color: #333; }
            h2 { color: #555; margin-top: 2em; }
            .svg-container { display: flex; justify-content: center; margin: 1em 0; }
            svg { max-width: 100%; height: auto; border: 1px solid #eee; border-radius: 8px; }
            .note { background: #f0f0f0; padding: 1em; border-radius: 8px; font-size: 0.9em; }
          </style>
        </head>
        <body>
          <h1>Dice Reference Vector Diagrams</h1>
          <div class="note">
            <p>Each <strong>colored arrow</strong> represents the gravity vector when
            that face is resting on the table. The vector points from the die center
            toward the downward face.</p>
            <p>When the accelerometer reads <code>(x, y, z)</code>, the
            <code>interpret()</code> function finds the closest reference vector
            (nearest neighbor by squared Euclidean distance) and returns the
            corresponding face number.</p>
            <p>Three views are shown for each die type to disambiguate vectors
            that overlap in a single projection:
            <strong>Isometric</strong> (3D perspective),
            <strong>Front</strong> (X-Z plane), and
            <strong>Top</strong> (X-Y plane).</p>
          </div>
    """)
    for name, _, title in svgs:
        svg_path = out_dir / f"{name}.svg"
        svg_content = svg_path.read_text(encoding="utf-8").strip()
        html += f'          <h2>{title}</h2>\n'
        html += f'          <div class="svg-container">\n'
        html += f'            {svg_content}\n'
        html += f'          </div>\n'
    html += textwrap.dedent("""\
        </body>
        </html>
    """)
    html_path = out_dir / "vector_diagrams.html"
    html_path.write_text(html, encoding="utf-8")
    print(f"\n  Open in browser: file://{html_path.resolve()}")


if __name__ == "__main__":
    main()
