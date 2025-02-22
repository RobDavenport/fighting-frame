mod api;
use api::*;

mod gameplay;

mod graphics;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn render() {}
