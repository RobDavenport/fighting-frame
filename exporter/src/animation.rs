use std::collections::HashMap;

use glam::{Mat4, Quat, Vec3};
use gltf::{Animation, Document, animation::util::ReadOutputs, buffer::Data};

struct Node {
    transform: Mat4,
    parent_index: Option<usize>,
}

pub fn build_animation_list(document: &Document, buffers: &[Data]) {
    let mut nodes = HashMap::new();

    for node in document.nodes() {
        let transform = node.transform().matrix();
        let transform = Mat4::from_cols_array_2d(&transform);
        nodes.insert(
            node.index(),
            Node {
                transform,
                parent_index: None,
            },
        );
    }

    for node in document.nodes() {
        let parent_index = node.index();
        for child in node.children() {
            nodes.get_mut(&child.index()).unwrap().parent_index = Some(parent_index);
        }
    }

    for animation in document.animations() {
        let anim_name = animation.name().unwrap();
        let node_keyframes = load_animation(&animation, buffers, &nodes);
    }
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
) -> Vec<Vec<Mat4>> {
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
    let mut keyframe_data = vec![vec![BoneChannelData::default(); num_keyframes]; nodes.len()];

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

    // Propagate missing channel values from previous frames.
    for node_index in 0..nodes.len() {
        {
            let data = &mut keyframe_data[0][node_index];
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

    let mut transform_data = vec![vec![Mat4::IDENTITY; num_keyframes]; nodes.len()];

    // Now convert them into transforms
    for (key_index, keyframe) in keyframe_data.iter().enumerate() {
        for (bone_index, channel_data) in keyframe.iter().enumerate() {
            transform_data[key_index][bone_index] = channel_data.as_matrix();
        }
    }

    // Now apply parent transforms
    for key_index in 0..transform_data.len() {
        for node_index in 0..transform_data[key_index].len() {
            let mut global_transform = Mat4::IDENTITY;
            let mut node_index = node_index;
            while let Some(parent) = nodes.get(&node_index) {
                if let Some(parent_index) = parent.parent_index {
                    global_transform = global_transform * transform_data[key_index][parent_index];
                    node_index = parent_index;
                } else {
                    break;
                }
            }

            transform_data[key_index][node_index] =
                global_transform * transform_data[key_index][node_index];
        }
    }

    transform_data
}
