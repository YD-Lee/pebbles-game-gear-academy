#![no_std]

use gstd::{msg, exec, debug};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    // todo
}

#[no_mangle]
extern "C" fn handle() {
    // todo
}

#[no_mangle]
extern "C" fn state() {
    // todo
}
