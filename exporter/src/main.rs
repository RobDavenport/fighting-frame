use std::collections::HashMap;

use gltf::{animation::util::ReadOutputs, buffer::Data, mesh::util::indices, Mesh, Primitive};

struct MeshData {
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>
}

impl MeshData {
    fn load(primitive: &Primitive<'_>, buffers: &[Data]) -> Self {
        let reader = primitive.reader(|b| Some(&buffers[b.index()]));

        let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
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
    let (document, buffers, images) = gltf::import("./exporter/assets/default char test.glb").unwrap();

    let mut meshes = HashMap::new();

    // Raw mesh data
    for mesh in document.meshes() {
        let name = mesh.name().unwrap().to_string();

        if mesh.primitives().len() > 1 {
            panic!("Primitives > 1")
        }

        let data = MeshData::load(&mesh.primitives().next().unwrap(), &buffers);
        meshes.insert(name, data);
    }

    //let mut animations = HashMap::new();

    // All named animations
    for animation in document.animations() {
        //let mut anim = HashMap::new();
        let name = animation.name();
        println!("importing animation: {name:?}");

        // Channels, like scale, rotation, translation, etc
        for channel in animation.channels() {
            let target = channel.target().node();
            let bone_name = target.name().unwrap();
            println!("Anim for {bone_name}.");

            let reader = channel.reader(|b| Some(&buffers[b.index()]));

            if let Some(i) = reader.read_inputs() {
                for input in i {
                    println!("{input}");
                }
            }

            if let Some(a) = reader.read_outputs() {
                match a {
                    ReadOutputs::Translations(iter) => {
                        println!("Translations: ");
                        for t in iter {
                            println!("{t:?}");
                        }
                    },
                    ReadOutputs::Rotations(rotations) => {
                        println!("Rotations:");
                        for r in rotations.into_f32() {
                            println!("{r:?}");
                        }
                    },
                    ReadOutputs::Scales(iter) => {
                        println!("Scales:");
                        for s in iter {
                            println!("{s:?}");
                        }
                    },
                    ReadOutputs::MorphTargetWeights(morph_target_weights) => todo!(),
                }
            }
        }
    }
}