use raylib::{RaylibHandle, ffi::KeyboardKey, prelude::RaylibDrawHandle};

use crate::raytris::gameplay::{Game, playfield::Playfield};

#[derive(Default)]
pub struct SinglePlayer {
  game: Game,
  undo_stack: Vec<Playfield>,
}

impl SinglePlayer {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn update(&mut self, _: &RaylibHandle) {}
  pub fn draw(&self, _: &RaylibDrawHandle) {}
  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
  }
}
