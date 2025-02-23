mod api;
use std::cell::RefCell;

use api::*;
use db::{CharacterDefinition, DEFAULT_CHARACTER};

mod gameplay;

mod graphics;

mod db;

mod static_data;

mod texture;

use glam::{Mat4, Vec3};

struct GameState {
    player_1: &'static CharacterDefinition,
    keyframe: usize,
    texture_id: i32,
}

thread_local! {
    static STATE: RefCell<GameState> = RefCell::new(GameState {
        player_1: DEFAULT_CHARACTER,
        keyframe: 0,
        texture_id: 0
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    let text = "Init Fighting Frame";
    let text2 = "Init Done";
    let texture = texture::generate_texture();
    unsafe {
        console_log(text.as_ptr(), text.len() as i32);
        STATE.with_borrow_mut(|state| {
            state.texture_id = load_texture(
                texture.as_ptr(),
                texture::TEXTURE_WIDTH as i32,
                texture::TEXTURE_HEIGHT as i32,
                1,
            );
        });

        for mesh in static_data::MESHES {
            load_static_mesh_indexed(
                mesh.vertices.as_ptr() as *const u8,
                mesh.vertices.len() as i32,
                mesh.indices.as_ptr() as *const u8,
                mesh.indices.len() as i32,
                6,
            );
        }
        console_log(text2.as_ptr(), text2.len() as i32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update() {
    STATE.with_borrow_mut(|state| {
        state.keyframe += 1;
        state.keyframe %= 4;
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn render() {
    let proj = Mat4::perspective_infinite_reverse_rh(71.0_f32.to_radians(), 16.0 / 9.0, 0.1);
    let pos = Vec3::new(0.0, 1.0, 5.0);
    let view = Mat4::look_to_rh(pos, Vec3::NEG_Z, Vec3::Y);

    unsafe {
        push_proj_matrix(&raw const proj as *const u8);
        push_view_matrix_pos(&raw const view as *const u8, &raw const pos as *const u8);

        STATE.with_borrow(|state| {
            set_texture(state.texture_id, 0, 0);
            let keyframe = state.keyframe;
            for i in 0..static_data::MESHES.len() {
                let model = static_data::GLOBAL_TRANSFORMS[i] * static_data::ANIMATIONS[0].1[keyframe][i];
                // static_data::GLOBAL_TRANSFORMS[i] * 
                //let model = static_data::ANIMATIONS[0].1[keyframe][i];
                push_model_matrix(
                    &raw const model as *const u8,
                );
                draw_static_mesh_indexed(i as i32);
            }
        })
    };
}
