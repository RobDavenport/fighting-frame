use std::collections::HashMap;

use glam::{Mat4, Quat, Vec3};
use gltf::{Animation, Document, animation::util::ReadOutputs, buffer::Data};
use shared::Trs;

use crate::OutAnimationData;

struct Node {
    transform: Mat4,
    parent_index: Option<usize>,
}

#[derive(PartialEq)]
struct NodeWithMesh {
    node_index: usize,
}

pub fn build_animation_list(document: &Document, buffers: &[Data]) -> Vec<OutAnimationData> {
    let mut nodes = HashMap::new();
    let mut nodes_with_meshes = Vec::new();

    for node in document.nodes() {
        let transform = Mat4::from_cols_array_2d(&node.transform().matrix());
        nodes.insert(
            node.index(),
            Node {
                transform,
                parent_index: None,
            },
        );

        if node.mesh().is_some() {
            nodes_with_meshes.push(NodeWithMesh {
                node_index: node.index(),
            })
        }
    }

    for node in document.nodes() {
        let parent_index = node.index();
        for child in node.children() {
            nodes.get_mut(&child.index()).unwrap().parent_index = Some(parent_index);
        }
    }

    let mut out = Vec::new();

    for animation in document.animations() {
        let anim_name = animation.name().unwrap();
        let node_keyframes = load_animation(&animation, buffers, &nodes);

        let mut anim_keyframes = Vec::new();
        for node in node_keyframes.iter() {
            let mut mesh_keyframes = Vec::new();
            for (node_index, transform) in node.iter().enumerate() {
                if nodes_with_meshes.contains(&NodeWithMesh { node_index }) {
                    mesh_keyframes.push(transform.clone());
                }
            }
            if !mesh_keyframes.is_empty() {
                anim_keyframes.push(mesh_keyframes);
            }
        }

        out.push(OutAnimationData {
            name: anim_name.to_string(),
            data: anim_keyframes,
        })
    }

    out
}

#[derive(Clone, Default)]
struct BoneChannelData {
    translation: Option<[f32; 3]>,
    rotation: Option<[f32; 4]>,
    scale: Option<[f32; 3]>,
}

impl BoneChannelData {
    fn as_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::from_array(self.scale.unwrap()),
            Quat::from_array(self.rotation.unwrap()),
            Vec3::from_array(self.translation.unwrap()),
        )
    }
}

fn load_animation(
    animation: &Animation,
    buffers: &[Data],
    nodes: &HashMap<usize, Node>,
) -> Vec<Vec<Trs>> {
    // Determine the maximum keyframe index.
    let mut max_frame = 0;
    for channel in animation.channels() {
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
    let mut keyframe_data = vec![vec![BoneChannelData::default(); nodes.len()]; num_keyframes];

    for channel in animation.channels() {
        let target = channel.target();
        let target_index = target.node().index();

        let reader = channel.reader(|b| Some(&buffers[b.index()]));
        let inputs = reader.read_inputs().unwrap();
        let outputs = reader.read_outputs().unwrap();

        match outputs {
            ReadOutputs::Translations(translations) => {
                for (time, translation) in inputs.zip(translations) {
                    let frame = time.floor() as usize;
                    keyframe_data[frame][target_index].translation = Some(translation);
                }
            }
            ReadOutputs::Rotations(rotations) => {
                for (time, rotation) in inputs.zip(rotations.into_f32()) {
                    let frame = time.floor() as usize;
                    keyframe_data[frame][target_index].rotation = Some(rotation);
                }
            }
            ReadOutputs::Scales(scales) => {
                for (time, scale) in inputs.zip(scales) {
                    let frame = time.floor() as usize;
                    keyframe_data[frame][target_index].scale = Some(scale);
                }
            }
            ReadOutputs::MorphTargetWeights(_weights) => {}
        }
    }

    // Before propagating missing channel values, set defaults using the static bind pose.
    for node_index in 0..nodes.len() {
        // Retrieve the node's static transform.
        let static_transform = nodes.get(&node_index).unwrap().transform;
        // Decompose the static transform into TRS components.
        // Note: Using glam's to_scale_rotation_translation method.
        let (static_scale, static_rot, static_trans) =
            static_transform.to_scale_rotation_translation();
        // Set the defaults for frame 0 if not animated.
        let data = &mut keyframe_data[0][node_index];
        if data.translation.is_none() {
            data.translation = Some(static_trans.to_array());
        }
        if data.rotation.is_none() {
            data.rotation = Some(static_rot.to_array());
        }
        if data.scale.is_none() {
            data.scale = Some(static_scale.to_array());
        }
    }

    // Now propagate missing channel values to subsequent frames.
    for node_index in 0..nodes.len() {
        for frame in 1..num_keyframes {
            let prev = keyframe_data[frame - 1][node_index].clone();
            let current = &mut keyframe_data[frame][node_index];
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

    let mut transform_data = vec![vec![Mat4::IDENTITY; nodes.len()]; num_keyframes];

    // Now convert them into transforms
    for (key_index, keyframe) in keyframe_data.iter().enumerate() {
        for (bone_index, channel_data) in keyframe.iter().enumerate() {
            transform_data[key_index][bone_index] = channel_data.as_matrix();
        }
    }

    // Now apply parent transforms
    for key_index in 0..transform_data.len() {
        // Iterate using a stable index 'i' for the current node.
        for i in 0..transform_data[key_index].len() {
            // Start with the local animated transform.
            let mut out = transform_data[key_index][i];
            // Use a separate variable for traversing up the hierarchy.
            let mut current = i;
            // Traverse up while there is a parent.
            while let Some(node) = nodes.get(&current) {
                if let Some(parent_index) = node.parent_index {
                    out = transform_data[key_index][parent_index] * out;
                    current = parent_index;
                } else {
                    break;
                }
            }
            // Store the final global transform back at the original node index.
            transform_data[key_index][i] = out;
        }
    }

    transform_data
        .into_iter()
        .map(|keyframe| keyframe.into_iter().map(Trs::from).collect::<Vec<Trs>>())
        .collect::<Vec<Vec<Trs>>>()
}
