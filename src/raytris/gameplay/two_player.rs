use raylib::{RaylibHandle, ffi::KeyboardKey, prelude::RaylibDrawHandle};

use crate::raytris::gameplay::Game;

#[derive(Default)]
pub struct TwoPlayer {
  game1: Game,
  game2: Game,
}

impl TwoPlayer {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn update(&mut self, _: &RaylibHandle) {}
  pub fn draw(&self, _: &RaylibDrawHandle) {}
  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
  }
}
