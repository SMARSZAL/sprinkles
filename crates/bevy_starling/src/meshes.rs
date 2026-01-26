use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub fn create_cylinder_mesh(
    top_radius: f32,
    bottom_radius: f32,
    height: f32,
    radial_segments: u32,
    rings: u32,
    cap_top: bool,
    cap_bottom: bool,
) -> Mesh {
    let radial_segments = radial_segments.max(3);
    let rings = rings.max(1);
    let half_height = height / 2.0;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // the side normal y component depends on the slope of the cylinder
    let side_normal_y = (bottom_radius - top_radius) / height;
    let side_normal_scale = 1.0 / (1.0 + side_normal_y * side_normal_y).sqrt();

    // generate side vertices (rings + 1 vertex rings from top to bottom)
    for ring in 0..=rings {
        let v = ring as f32 / rings as f32;
        let y = half_height - height * v;
        let radius = top_radius + (bottom_radius - top_radius) * v;

        for segment in 0..=radial_segments {
            let u = segment as f32 / radial_segments as f32;
            let theta = u * std::f32::consts::TAU;
            let (sin_theta, cos_theta) = theta.sin_cos();

            let x = cos_theta * radius;
            let z = sin_theta * radius;

            positions.push([x, y, z]);

            let nx = cos_theta * side_normal_scale;
            let ny = side_normal_y * side_normal_scale;
            let nz = sin_theta * side_normal_scale;
            normals.push([nx, ny, nz]);

            uvs.push([u, v]);
        }
    }

    // generate side indices (counter-clockwise winding for front faces)
    let verts_per_ring = radial_segments + 1;
    for ring in 0..rings {
        for segment in 0..radial_segments {
            let top_left = ring * verts_per_ring + segment;
            let top_right = ring * verts_per_ring + segment + 1;
            let bottom_left = (ring + 1) * verts_per_ring + segment;
            let bottom_right = (ring + 1) * verts_per_ring + segment + 1;

            indices.push(top_left);
            indices.push(top_right);
            indices.push(bottom_left);

            indices.push(top_right);
            indices.push(bottom_right);
            indices.push(bottom_left);
        }
    }

    // generate top cap
    if cap_top && top_radius > 0.0 {
        let center_index = positions.len() as u32;

        positions.push([0.0, half_height, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5, 0.5]);

        for segment in 0..=radial_segments {
            let u = segment as f32 / radial_segments as f32;
            let theta = u * std::f32::consts::TAU;
            let (sin_theta, cos_theta) = theta.sin_cos();

            let x = cos_theta * top_radius;
            let z = sin_theta * top_radius;

            positions.push([x, half_height, z]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([cos_theta * 0.5 + 0.5, sin_theta * 0.5 + 0.5]);
        }

        for segment in 0..radial_segments {
            let first = center_index + 1 + segment;
            let second = center_index + 1 + segment + 1;
            indices.push(center_index);
            indices.push(second);
            indices.push(first);
        }
    }

    // generate bottom cap
    if cap_bottom && bottom_radius > 0.0 {
        let center_index = positions.len() as u32;

        positions.push([0.0, -half_height, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5, 0.5]);

        for segment in 0..=radial_segments {
            let u = segment as f32 / radial_segments as f32;
            let theta = u * std::f32::consts::TAU;
            let (sin_theta, cos_theta) = theta.sin_cos();

            let x = cos_theta * bottom_radius;
            let z = sin_theta * bottom_radius;

            positions.push([x, -half_height, z]);
            normals.push([0.0, -1.0, 0.0]);
            uvs.push([cos_theta * 0.5 + 0.5, sin_theta * 0.5 + 0.5]);
        }

        for segment in 0..radial_segments {
            let first = center_index + 1 + segment;
            let second = center_index + 1 + segment + 1;
            indices.push(center_index);
            indices.push(first);
            indices.push(second);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
