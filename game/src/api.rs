#[allow(unused)]
unsafe extern "C" {
    // Data
    pub fn height() -> i32;
    pub fn width() -> i32;
    pub fn fps() -> i32;
    pub fn frame_time() -> f32;

    pub fn push_audio(buffer_ptr: *const u8, buffer_len: i32, channel_count: i32, sample_rate: i32);

    // Draw 3d
    pub fn draw_tri_list(data_ptr: *const u8, len: i32, pipeline: i32);
    pub fn draw_tri_list_indexed(
        data_ptr: *const u8,
        data_len: i32,
        index_ptr: *const u8,
        index_len: i32,
        pipeline: i32,
    );
    pub fn push_model_matrix(mat_ptr: *const u8);
    pub fn push_proj_matrix(proj_ptr: *const u8);
    pub fn push_view_matrix_pos(view_ptr: *const u8, pos_ptr: *const u8);
    pub fn draw_static_mesh(id: i32);
    pub fn draw_static_mesh_indexed(id: i32);
    pub fn draw_sprite(id: i32);
    pub fn set_texture(id: i32, layer: i32, blend: i32);
    pub fn set_matcap(id: i32, layer: i32, blend: i32);
    pub fn load_texture(data_ptr: *const u8, width: i32, height: i32, has_alpha: i32) -> i32;
    pub fn load_static_mesh(data_ptr: *const u8, data_len: i32, pipeline: i32) -> i32;
    pub fn load_static_mesh_indexed(
        data_ptr: *const u8,
        data_len: i32,
        index_ptr: *const u8,
        index_len: i32,
        pipeline: i32,
    ) -> i32;

    // Input
    pub fn button_a_pressed(player_id: i32) -> i32;
    pub fn button_a_released(player_id: i32) -> i32;
    pub fn button_a_held(player_id: i32) -> i32;
    pub fn button_b_pressed(player_id: i32) -> i32;
    pub fn button_b_released(player_id: i32) -> i32;
    pub fn button_b_held(player_id: i32) -> i32;
    pub fn button_c_pressed(player_id: i32) -> i32;
    pub fn button_c_released(player_id: i32) -> i32;
    pub fn button_c_held(player_id: i32) -> i32;
    pub fn button_d_pressed(player_id: i32) -> i32;
    pub fn button_d_released(player_id: i32) -> i32;
    pub fn button_d_held(player_id: i32) -> i32;
    pub fn button_up_pressed(player_id: i32) -> i32;
    pub fn button_up_released(player_id: i32) -> i32;
    pub fn button_up_held(player_id: i32) -> i32;
    pub fn button_down_pressed(player_id: i32) -> i32;
    pub fn button_down_released(player_id: i32) -> i32;
    pub fn button_down_held(player_id: i32) -> i32;
    pub fn button_left_pressed(player_id: i32) -> i32;
    pub fn button_left_released(player_id: i32) -> i32;
    pub fn button_left_held(player_id: i32) -> i32;
    pub fn button_right_pressed(player_id: i32) -> i32;
    pub fn button_right_released(player_id: i32) -> i32;
    pub fn button_right_held(player_id: i32) -> i32;
    pub fn button_start_pressed(player_id: i32) -> i32;
    pub fn button_start_released(player_id: i32) -> i32;
    pub fn button_start_held(player_id: i32) -> i32;
    pub fn button_select_pressed(player_id: i32) -> i32;
    pub fn button_select_released(player_id: i32) -> i32;
    pub fn button_select_held(player_id: i32) -> i32;
    pub fn button_left_shoulder_pressed(player_id: i32) -> i32;
    pub fn button_left_shoulder_released(player_id: i32) -> i32;
    pub fn button_left_shoulder_held(player_id: i32) -> i32;
    pub fn button_right_shoulder_pressed(player_id: i32) -> i32;
    pub fn button_right_shoulder_released(player_id: i32) -> i32;
    pub fn button_right_shoulder_held(player_id: i32) -> i32;
    pub fn button_left_stick_pressed(player_id: i32) -> i32;
    pub fn button_left_stick_released(player_id: i32) -> i32;
    pub fn button_left_stick_held(player_id: i32) -> i32;
    pub fn button_right_stick_pressed(player_id: i32) -> i32;
    pub fn button_right_stick_released(player_id: i32) -> i32;
    pub fn button_right_stick_held(player_id: i32) -> i32;
    pub fn button_left_trigger_pressed(player_id: i32) -> i32;
    pub fn button_left_trigger_released(player_id: i32) -> i32;
    pub fn button_left_trigger_held(player_id: i32) -> i32;
    pub fn button_right_trigger_pressed(player_id: i32) -> i32;
    pub fn button_right_trigger_released(player_id: i32) -> i32;
    pub fn button_right_trigger_held(player_id: i32) -> i32;
    pub fn analog_left_x(player_id: i32) -> f32;
    pub fn analog_left_y(player_id: i32) -> f32;
    pub fn analog_right_x(player_id: i32) -> f32;
    pub fn analog_right_y(player_id: i32) -> f32;
    pub fn trigger_left(player_id: i32) -> f32;
    pub fn trigger_right(player_id: i32) -> f32;

    pub fn mouse_left_pressed(player_id: i32) -> i32;
    pub fn mouse_left_released(player_id: i32) -> i32;
    pub fn mouse_left_held(player_id: i32) -> i32;
    pub fn mouse_right_pressed(player_id: i32) -> i32;
    pub fn mouse_right_released(player_id: i32) -> i32;
    pub fn mouse_right_held(player_id: i32) -> i32;
    pub fn mouse_middle_pressed(player_id: i32) -> i32;
    pub fn mouse_middle_released(player_id: i32) -> i32;
    pub fn mouse_middle_held(player_id: i32) -> i32;

    pub fn mouse_x_pos(player_id: i32) -> i32;
    pub fn mouse_y_pos(player_id: i32) -> i32;
    pub fn mouse_x_delta(player_id: i32) -> i32;
    pub fn mouse_y_delta(player_id: i32) -> i32;

    pub fn mouse_wheel_up(player_id: i32) -> i32;
    pub fn mouse_wheel_down(player_id: i32) -> i32;
    pub fn mouse_wheel_left(player_id: i32) -> i32;
    pub fn mouse_wheel_right(player_id: i32) -> i32;
    pub fn lock_mouse(locked: i32);

    pub fn raw_input_state(player_id: i32) -> i64;
    pub fn raw_mouse_state(player_id: i32) -> i64;

    // Text
    pub fn console_log(text_ptr: *const u8, len: i32);
    pub fn console_log_utf16(text_ptr: *const u8, len: i32);

    // Random
    pub fn set_seed(seed: i32);
    pub fn random_int_range(min: i32, max: i32) -> i32;
    pub fn random_float() -> f32;
    pub fn random_float_range(min: f32, max: f32) -> f32;
}
