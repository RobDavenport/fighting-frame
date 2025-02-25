mod api;
use std::cell::RefCell;

use api::*;
use db::{CharacterDefinition, DEFAULT_CHARACTER};

mod gameplay;

mod graphics;

mod db;

mod static_data;

mod texture;

use glam::{Mat4, Quat, Vec3};

const KEYFRAME_SPEED: usize = 4;

struct GameState {
    player_1: &'static CharacterDefinition,
    keyframe: usize,
    texture_id: i32,
    matcap_id: i32,
}

thread_local! {
    static STATE: RefCell<GameState> = RefCell::new(GameState {
        player_1: &DEFAULT_CHARACTER,
        keyframe: 0,
        texture_id: 0,
        matcap_id: 0,
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    let text = "Init Fighting Frame";
    let text2 = "Init Done";
    let texture = texture::generate_texture();
    let matcap = texture::generate_matcap_bytes(256);
    unsafe {
        console_log(text.as_ptr(), text.len() as i32);
        STATE.with_borrow_mut(|state| {
            state.texture_id = load_texture(
                texture.as_ptr(),
                texture::TEXTURE_WIDTH as i32,
                texture::TEXTURE_HEIGHT as i32,
                1,
            );
            state.matcap_id = load_texture(matcap.as_ptr(), 256, 256, 1);

            for mesh in state.player_1.graphics.meshes {
                load_static_mesh_indexed(
                    mesh.vertices.as_ptr() as *const u8,
                    mesh.vertices.len() as i32,
                    mesh.indices.as_ptr() as *const u8,
                    mesh.indices.len() as i32,
                    6,
                );
            }
        });

        console_log(text2.as_ptr(), text2.len() as i32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update() {
    STATE.with_borrow_mut(|state| {
        state.keyframe += 1;
        state.keyframe %= 4 * KEYFRAME_SPEED;
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn render() {
    let proj = Mat4::perspective_infinite_reverse_rh(71.0_f32.to_radians(), 16.0 / 9.0, 0.1);
    let pos = Vec3::new(5.0, 1.0, 0.0);
    let view = Mat4::look_to_rh(pos, Vec3::NEG_X, Vec3::Y);

    let p1 = Mat4::from_translation(Vec3::new(0.0, 0.0, 2.0));
    let p2 = Mat4::from_scale_rotation_translation(
        Vec3::new(1.0, 1.0, -1.0),
        Quat::IDENTITY,
        Vec3::new(0.0, 0.0, -2.0),
    );

    unsafe {
        push_proj_matrix(&raw const proj as *const u8);
        push_view_matrix_pos(&raw const view as *const u8, &raw const pos as *const u8);

        STATE.with_borrow(|state| {
            set_texture(state.texture_id, 0, 0);
            let keyframe = state.keyframe / KEYFRAME_SPEED;
            let key_mod = state.keyframe % KEYFRAME_SPEED;
            let s = key_mod as f32 / KEYFRAME_SPEED as f32;

            for i in 0..state.player_1.graphics.meshes.len() {
                let model = p1
                    * state.player_1.graphics.animations[1]
                        .blend(keyframe, i, s)
                        .matrix();
                push_model_matrix(&raw const model as *const u8);
                draw_static_mesh_indexed(i as i32);
            }

            set_winding_order(1);
            set_matcap(state.matcap_id, 1, 3);

            for i in 0..state.player_1.graphics.meshes.len() {
                let model = p2
                    * state.player_1.graphics.animations[1]
                        .blend(keyframe, i, s)
                        .matrix();
                push_model_matrix(&raw const model as *const u8);
                draw_static_mesh_indexed(i as i32);
            }
        })
    };
}
