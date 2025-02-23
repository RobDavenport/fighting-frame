use std::collections::HashMap;

use glam::Mat4;
use gltf::{Animation, Document};

struct Node {
    transform: Mat4,
    parent_index: Option<usize>,
}

fn get_global_transform(mut node_index: usize, nodes: &HashMap<usize, Node>) -> Mat4 {
    let mut out = Mat4::IDENTITY;

    while let Some(node) = nodes.get(&node_index) {
        out = node.transform * out;
        if let Some(parent_index) = node.parent_index {
            node_index = parent_index;
        } else {
            break;
        }
    }

    out
}

pub fn build_animation_list(document: &Document) {
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
}

fn load_animation(animation: &Animation, nodes: &HashMap<usize, Node>) {}
