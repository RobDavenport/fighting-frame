mod animation;
mod source_writer;

use glam::{Mat4, Quat, Vec3};
use gltf::{Primitive, buffer::Data};
use shared::Trs;
use std::collections::HashMap;

/// Raw mesh data loaded from a glTF primitive.
struct MeshData {
    indices: Vec<u16>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl MeshData {
    fn load(primitive: &Primitive<'_>, buffers: &[Data]) -> Self {
        let reader = primitive.reader(|b| Some(&buffers[b.index()]));

        let indices: Vec<u16> = reader
            .read_indices()
            .unwrap()
            .into_u32()
            .map(|n| u16::try_from(n).unwrap())
            .collect();
        let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
        let normals: Vec<[f32; 3]> = reader.read_normals().unwrap().collect();
        let uvs: Vec<[f32; 2]> = reader.read_tex_coords(0).unwrap().into_f32().collect();

        Self {
            indices,
            positions,
            normals,
            uvs,
        }
    }
}

fn main() {
    // Import the glTF file.
    let (document, buffers, _images) =
        gltf::import("./exporter/assets/default char test.glb").unwrap();

    // --- Import Meshes ---
    let mut meshes = HashMap::new();
    let mut mesh_names = Vec::new();
    let mut mesh_to_index = HashMap::new();
    let mut child_to_parent = HashMap::new();
    let mut mesh_to_node_index = HashMap::new();

    for node in document.nodes() {
        let (t, r, s) = node.transform().decomposed();
        let parent_index = node.index();
        let transform = Mat4::from_scale_rotation_translation(
            Vec3::from(s),
            Quat::from_xyzw(r[0], r[1], r[2], r[3]),
            Vec3::from(t),
        );

        if let Some(mesh) = node.mesh() {
            if let Some(mesh_name) = mesh.name() {
                mesh_to_node_index.insert(mesh_name.to_string(), node.index());
            }
        }

        for child in node.children() {
            let child_index = child.index();
            child_to_parent.insert(child_index, (parent_index, transform));
        }
    }

    for (counter, mesh) in document.meshes().enumerate() {
        let name = mesh.name().unwrap().to_string();
        println!("found mesh: {name}");

        if mesh.primitives().len() > 1 {
            panic!("Primitives > 1");
        }

        let data = MeshData::load(&mesh.primitives().next().unwrap(), &buffers);
        meshes.insert(name.clone(), data);
        mesh_names.push(name.clone());
        mesh_to_index.insert(name, counter);
    }

    // --- Process Animations ---
    let animations = animation::build_animation_list(&document, &buffers);

    let mut out_mesh = Vec::new();

    for mesh_name in &mesh_names {
        let mesh = meshes.get(mesh_name).unwrap();

        let mut vertices = Vec::new();
        for i in 0..mesh.positions.len() {
            let pos = mesh.positions[i];
            let uv = mesh.uvs[i];
            let norm = mesh.normals[i];

            vertices.extend_from_slice(&pos);
            vertices.extend_from_slice(&uv);
            vertices.extend_from_slice(&norm);
        }

        out_mesh.push(OutMeshData {
            vertices,
            indices: mesh.indices.clone(),
        })
    }

    for animation in animations.iter() {
        println!("Found animation: {}", animation.name);
    }

    source_writer::generate_character_data_source(
        &OutCharacterData {
            mesh: out_mesh,
            animation_data: animations,
        },
        "static_data.rs",
    )
    .unwrap();
}

struct OutCharacterData {
    mesh: Vec<OutMeshData>,
    animation_data: Vec<OutAnimationData>,
}

struct OutMeshData {
    vertices: Vec<f32>,
    indices: Vec<u16>,
}

struct OutAnimationData {
    name: String,
    data: Vec<Vec<Trs>>,
}
