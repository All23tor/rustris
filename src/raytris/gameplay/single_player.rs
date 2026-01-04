use raylib::{RaylibHandle, ffi::KeyboardKey, prelude::RaylibDrawHandle};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails, Game, PLAYFIELD_VECTOR,
    playfield::{self, Playfield},
    screen_vector,
  },
  settings::handling::HandlingSettings,
};

pub struct SinglePlayer {
  game: Game,
  undo_stack: Vec<Playfield>,
}

// TODO: implement saving and loading game state
impl SinglePlayer {
  const KEYBOARD_CONTROLS: Controller = Controller {
    restart: |rl| rl.is_key_pressed(KeyboardKey::KEY_R),
    swap: |rl| rl.is_key_pressed(KeyboardKey::KEY_C),
    left: |rl| rl.is_key_pressed(KeyboardKey::KEY_LEFT),
    right: |rl| rl.is_key_pressed(KeyboardKey::KEY_RIGHT),
    left_das: |rl| rl.is_key_down(KeyboardKey::KEY_LEFT),
    right_das: |rl| rl.is_key_down(KeyboardKey::KEY_RIGHT),
    clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_UP),
    counter_clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_Z),
    one_eighty: |rl| rl.is_key_pressed(KeyboardKey::KEY_A),
    hard_drop: |rl| rl.is_key_pressed(KeyboardKey::KEY_SPACE),
    soft_drop: |rl| rl.is_key_pressed(KeyboardKey::KEY_DOWN),
    undo: |rl| {
      rl.is_key_pressed(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_Z)
    },
    pause: |rl| rl.is_key_pressed(KeyboardKey::KEY_ENTER),
    quit: |rl| rl.is_key_pressed(KeyboardKey::KEY_ESCAPE),
  };

  fn drawing_details(rl: &RaylibHandle) -> DrawingDetails {
    let block_length = DrawingDetails::HEIGHT_SCALE_FACTOR * rl.get_screen_height() as f32
      / playfield::VISIBLE_HEIGHT as f32;
    let position = (screen_vector(rl) - PLAYFIELD_VECTOR * block_length) / 2.0;
    DrawingDetails::new(block_length, position)
  }

  pub fn new(settings: HandlingSettings, rl: &RaylibHandle) -> Self {
    let game = Game::new(Self::drawing_details(rl), Self::KEYBOARD_CONTROLS, settings);
    let undo_stack = vec![game.playfield.clone()];
    Self { game, undo_stack }
  }
  pub fn update(&mut self, rl: &RaylibHandle) {
    if (self.game.controller.undo)(rl)
      && let Some(top) = self.undo_stack.pop()
    {
      self.game.playfield = top;
      if self.undo_stack.is_empty() {
        self.undo_stack.push(self.game.playfield.clone());
        return;
      }
    }

    if self.game.update(rl) {
      self.undo_stack.push(self.game.playfield.clone());
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    self.game.draw(rld);
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    (self.game.controller.quit)(rl) && (self.game.pause || self.game.playfield.lost())
  }
}
