mod animation;

use glam::{Mat4, Quat, Vec3};
use gltf::{Primitive, animation::util::ReadOutputs, buffer::Data};
use std::collections::HashMap;
use std::fs;

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

/// Intermediate structure to store optional transform components for each bone.
#[derive(Clone, Default)]
struct BoneChannelData {
    translation: Option<[f32; 3]>,
    rotation: Option<[f32; 4]>,
    scale: Option<[f32; 3]>,
}

/// A keyframe is a vector of final transformation matrices (one per bone).
type Keyframe = Vec<Mat4>;

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
    let mut animations: HashMap<String, Vec<Keyframe>> = HashMap::new();
    for animation in document.animations() {
        let anim_name = animation.name().unwrap().to_string();
        println!("importing animation: {:?}", anim_name);

        let channels: Vec<_> = animation.channels().collect();

        // Determine the maximum keyframe index.
        let mut max_frame = 0;
        for channel in &channels {
            let reader = channel.reader(|b| Some(&buffers[b.index()]));
            if let Some(inputs_iter) = reader.read_inputs() {
                for time in inputs_iter {
                    let frame = time.floor() as usize;
                    if frame > max_frame {
                        max_frame = frame;
                    }
                }
            }
        }
        let num_keyframes = max_frame + 1;

        // Create a 2D vector for keyframe data.
        let mut keyframe_data: Vec<Vec<BoneChannelData>> =
            vec![vec![BoneChannelData::default(); mesh_names.len()]; num_keyframes];

        // Process each channel.
        for channel in channels {
            let target = channel.target().node();

            // Find the target by checking if one of its children has a mesh with a matching name.
            let target_name = target.children().find_map(|c| {
                if let Some(mesh) = c.mesh() {
                    mesh.name().map(|s| s.to_string())
                } else {
                    None
                }
            });

            if let Some(bone_name) = target_name {
                if let Some(&bone_index) = mesh_to_index.get(&bone_name) {
                    let reader = channel.reader(|b| Some(&buffers[b.index()]));
                    let inputs = reader.read_inputs().unwrap();

                    match reader.read_outputs().unwrap() {
                        ReadOutputs::Translations(translations) => {
                            for (time, translation) in inputs.zip(translations) {
                                let frame = time.floor() as usize;
                                keyframe_data[frame][bone_index].translation = Some(translation);
                            }
                        }
                        ReadOutputs::Rotations(rotations) => {
                            for (time, rotation) in inputs.zip(rotations.into_f32()) {
                                let frame = time.floor() as usize;
                                keyframe_data[frame][bone_index].rotation = Some(rotation);
                            }
                        }
                        ReadOutputs::Scales(scales) => {
                            for (time, scale) in inputs.zip(scales) {
                                let frame = time.floor() as usize;
                                keyframe_data[frame][bone_index].scale = Some(scale);
                            }
                        }
                        ReadOutputs::MorphTargetWeights(_weights) => {}
                    }
                }
            }
        }

        // Propagate missing channel values from previous frames.
        for bone_index in 0..mesh_names.len() {
            {
                let data = &mut keyframe_data[0][bone_index];
                if data.translation.is_none() {
                    data.translation = Some([0.0, 0.0, 0.0]);
                }
                if data.rotation.is_none() {
                    data.rotation = Some([0.0, 0.0, 0.0, 1.0]);
                }
                if data.scale.is_none() {
                    data.scale = Some([1.0, 1.0, 1.0]);
                }
            }
            for frame in 1..num_keyframes {
                let prev = keyframe_data[frame - 1][bone_index].clone();
                let current = &mut keyframe_data[frame][bone_index];
                if current.translation.is_none() {
                    current.translation = Some(prev.translation.unwrap());
                }
                if current.rotation.is_none() {
                    current.rotation = Some(prev.rotation.unwrap());
                }
                if current.scale.is_none() {
                    current.scale = Some(prev.scale.unwrap());
                }
            }
        }

        // Convert channel data into final keyframe matrices.
        let animation_keyframes: Vec<Keyframe> = keyframe_data
            .into_iter()
            .enumerate()
            .map(|(mesh_index, bone_channels)| {
                bone_channels
                    .into_iter()
                    .map(|channel| {
                        let translation = Vec3::from(channel.translation.unwrap());
                        let rotation_arr = channel.rotation.unwrap();
                        let rotation = Quat::from_xyzw(
                            rotation_arr[0],
                            rotation_arr[1],
                            rotation_arr[2],
                            rotation_arr[3],
                        );
                        let scale = Vec3::from(channel.scale.unwrap());
                        let local_transform =
                            Mat4::from_scale_rotation_translation(scale, rotation, translation);

                        let mut mesh_node_index =
                            *mesh_to_node_index.get(&mesh_names[mesh_index]).unwrap();
                        let mut global_transform = Mat4::IDENTITY;
                        while let Some((parent_index, parent_transform)) =
                            child_to_parent.get(&mesh_node_index)
                        {
                            global_transform = global_transform * *parent_transform;
                            mesh_node_index = *parent_index;
                        }

                        global_transform * local_transform
                    })
                    .collect()
            })
            .collect();
        animations.insert(anim_name, animation_keyframes);
    }

    // --- Output to a Rust source file ---
    let mut output = String::new();

    // File header.
    output
        .push_str("// This file is generated by the glTF importer tool. Do not edit manually.\n\n");
    output.push_str("use glam::Mat4;\n\n");

    // MeshData struct.
    output.push_str("pub struct MeshData {\n");
    output.push_str("    pub vertices: &'static [f32],\n");
    output.push_str("    pub indices: &'static [u16],\n");
    output.push_str("}\n\n");

    // Write meshes.
    output.push_str("pub static MESHES: &[MeshData] = &[\n");
    for mesh_name in &mesh_names {
        let mesh = meshes.get(mesh_name).unwrap();
        output.push_str("    MeshData {\n");
        output.push_str("        vertices: &[\n");
        for i in 0..mesh.positions.len() {
            let pos = mesh.positions[i];
            let uv = mesh.uvs[i];
            let norm = mesh.normals[i];
            output.push_str(&format!(
                "            {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6},\n",
                pos[0], pos[1], pos[2], uv[0], uv[1], norm[0], norm[1], norm[2]
            ));
        }
        output.push_str("        ],\n");
        output.push_str("        indices: &[\n");
        for idx in &mesh.indices {
            output.push_str(&format!("            {},\n", idx));
        }
        output.push_str("        ],\n");
        output.push_str("    },\n");
    }
    output.push_str("];\n\n");

    // Write animations.
    output.push_str("pub static ANIMATIONS: &[(&str, &[&[Mat4]])] = &[\n");
    for (anim_name, keyframes) in &animations {
        output.push_str(&format!("    (\"{}\", &[\n", anim_name));
        for keyframe in keyframes {
            output.push_str("        &[\n");
            for mat in keyframe {
                let arr = mat.to_cols_array();
                output.push_str(&format!(
                    "            Mat4::from_cols_array(&[{:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}, {:.6}]),\n",
                    arr[0], arr[1], arr[2], arr[3],
                    arr[4], arr[5], arr[6], arr[7],
                    arr[8], arr[9], arr[10], arr[11],
                    arr[12], arr[13], arr[14], arr[15],
                ));
            }
            output.push_str("        ],\n");
        }
        output.push_str("    ]),\n");
    }
    output.push_str("];\n");

    fs::write("static_data.rs", output).expect("Failed to write static data file");
    println!("Static data written to static_data.rs");

    // Debug print: number of keyframes per animation.
    for (anim_name, keyframes) in animations.iter() {
        println!(
            "Imported Animation '{}' has {} keyframes.",
            anim_name,
            keyframes.len()
        );
    }
}
