use crate::math::{Vec3};

/// One vertex as sent to the GPU:
/// position (xyz) + texture coords (uv) + normal (xyz) + face_id (for coloring)
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position:  [f32; 3],
    pub tex_coord: [f32; 2],
    pub normal:    [f32; 3],
    pub face_id:   f32,   // used in shader to assign per-face color
}

/// Everything the renderer needs from an OBJ file
pub struct Mesh {
    pub vertices: Vec<Vertex>,  // flat list ready to upload to GPU
    pub center:   Vec3,         // bounding box center (for rotation origin)
}

pub fn parse(path: &str) -> Result<Mesh, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {}: {}", path, e))?;

    // Raw data from the file
    let mut positions:  Vec<[f32; 3]> = Vec::new();
    let mut tex_coords: Vec<[f32; 2]> = Vec::new();
    let mut normals:    Vec<[f32; 3]> = Vec::new();

    // Final flat vertex list (triangulated)
    let mut vertices: Vec<Vertex> = Vec::new();

    let mut face_id: f32 = 0.0;

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        match tokens.next() {

            // ── Vertex position ───────────────────────────────────────────────
            Some("v") => {
                let coords: Vec<f32> = tokens
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if coords.len() >= 3 {
                    positions.push([coords[0], coords[1], coords[2]]);
                }
            }

            // ── Texture coordinate ────────────────────────────────────────────
            Some("vt") => {
                let coords: Vec<f32> = tokens
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if coords.len() >= 2 {
                    tex_coords.push([coords[0], coords[1]]);
                }
            }

            // ── Normal vector ─────────────────────────────────────────────────
            Some("vn") => {
                let coords: Vec<f32> = tokens
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if coords.len() >= 3 {
                    normals.push([coords[0], coords[1], coords[2]]);
                }
            }

            // ── Face ──────────────────────────────────────────────────────────
            Some("f") => {
                face_id += 1.0;

                // Parse all vertex references on this face line
                // Each token is one of:  "v"  "v/vt"  "v//vn"  "v/vt/vn"
                let face_verts: Vec<Vertex> = tokens
                    .map(|token| parse_face_vertex(
                        token,
                        &positions,
                        &tex_coords,
                        &normals,
                        face_id,
                    ))
                    .collect();

                // Triangulate: fan triangulation from first vertex
                // Triangle 0: [0, 1, 2]
                // Triangle 1: [0, 2, 3]
                // Triangle n: [0, n, n+1]
                // This works for convex polygons (handles quads and n-gons)
                for i in 1..(face_verts.len() as isize - 1) {
                    vertices.push(face_verts[0]);
                    vertices.push(face_verts[i as usize]);
                    vertices.push(face_verts[i as usize + 1]);
                }
            }

            // Ignore all other lines (mtllib, usemtl, s, g, o, etc.)
            _ => {}
        }
    }

    if vertices.is_empty() {
        return Err(format!("No geometry found in {}", path));
    }

    // Compute bounding box center so we can rotate around it
    let center = compute_center(&positions);

    // Shift all vertices so the center is at origin (0,0,0)
    // This ensures rotation happens around the object's center
    let vertices = vertices.into_iter().map(|mut v| {
        v.position[0] -= center.x;
        v.position[1] -= center.y;
        v.position[2] -= center.z;
        v
    }).collect();

    println!(
        "OBJ loaded: {} vertices, {} faces, center={:?}",
        positions.len(), face_id as u32, center
    );

    Ok(Mesh { vertices, center })
}

/// Parse one face vertex token like "1", "1/2", "1//3", "1/2/3"
/// OBJ indices are 1-based → subtract 1
fn parse_face_vertex(
    token: &str,
    positions:  &[[f32; 3]],
    tex_coords: &[[f32; 2]],
    normals:    &[[f32; 3]],
    face_id: f32,
) -> Vertex {
    let parts: Vec<&str> = token.split('/').collect();

    // Position index (always present)
    let pos_idx = parts[0].parse::<usize>().unwrap_or(1) - 1;
    let position = positions.get(pos_idx).copied().unwrap_or([0.0, 0.0, 0.0]);

    // Texture coord index (optional — part[1] may be empty)
    let tex_coord = if parts.len() > 1 && !parts[1].is_empty() {
        let tc_idx = parts[1].parse::<usize>().unwrap_or(1) - 1;
        tex_coords.get(tc_idx).copied().unwrap_or([0.0, 0.0])
    } else {
        [0.0, 0.0]
    };

    // Normal index (optional)
    let normal = if parts.len() > 2 && !parts[2].is_empty() {
        let n_idx = parts[2].parse::<usize>().unwrap_or(1) - 1;
        normals.get(n_idx).copied().unwrap_or([0.0, 1.0, 0.0])
    } else {
        [0.0, 1.0, 0.0]  // default up normal
    };

    Vertex { position, tex_coord, normal, face_id }
}

/// Compute the center of the bounding box of all vertex positions
fn compute_center(positions: &[[f32; 3]]) -> Vec3 {
    if positions.is_empty() {
        return Vec3::zero();
    }
    let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

    for p in positions {
        if p[0] < min.x { min.x = p[0]; }
        if p[1] < min.y { min.y = p[1]; }
        if p[2] < min.z { min.z = p[2]; }
        if p[0] > max.x { max.x = p[0]; }
        if p[1] > max.y { max.y = p[1]; }
        if p[2] > max.z { max.z = p[2]; }
    }

    Vec3::new(
        (min.x + max.x) / 2.0,
        (min.y + max.y) / 2.0,
        (min.z + max.z) / 2.0,
    )
}