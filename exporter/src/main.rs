use gltf::Mesh;

fn main() {
    let (document, buffers, images) = gltf::import("./exporter/assets/default char test.glb").unwrap();

    // Raw mesh data
    for mesh in document.meshes() {
        let name = mesh.name();
        println!("found mesh: {name:?}");

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|b| Some(&buffers[b.index()]));
            println!("\nPositions:");
            if let Some(iter) = reader.read_positions() {
                for position in iter {
                    println!("{position:?}")
                }
            }
            println!("\nNormals");
            if let Some(iter) = reader.read_normals() {
                for normal in iter {
                    println!("{normal:?}")
                }
            }

            println!("\nUVs");
            if let Some(iter) = reader.read_tex_coords(0) {
                for uv in iter.into_f32() {
                    println!("{uv:?}")
                }
            }
        }
    }

    // All named animations
    for anim in document.animations() {
        let name = anim.name();
        println!("importing animation: {name:?}");

        // Channels, like scale, rotation, translation, etc
        for channel in anim.channels() {
            let target = channel.target().node().name();
            let prop = channel.target().property();
            println!("found channel: {prop:?} for {:?}, ", target);
            for child in channel.target().node().children() {
                if let Some(mesh) = child.mesh() {
                    let name = child.name();
                    let mesh_name = mesh.name();
                    println!("Found child {name:?} of mesh {mesh_name:?}")
                }
            }
        }
    }
}