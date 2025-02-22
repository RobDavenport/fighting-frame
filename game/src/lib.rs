mod api;
use std::cell::RefCell;

use api::*;
use db::{CharacterDefinition, DEFAULT_CHARACTER};

mod gameplay;

mod graphics;

mod db;

struct GameState {
    player_1: &'static CharacterDefinition,
}

thread_local! {
    static STATE: RefCell<GameState> = RefCell::new(GameState {
        player_1: DEFAULT_CHARACTER,
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    let text = "Init Fighting Frame";
    unsafe { console_log(text.as_ptr(), text.len() as i32) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn render() {}
