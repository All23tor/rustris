use raylib::{RaylibHandle, consts::KeyboardKey, math::Vector2, prelude::RaylibDrawHandle};

use crate::raytris::{
  gameplay::{Controller, DrawingDetails, Game, PLAYFIELD_VECTOR, playfield, screen_vector},
  settings::handling::HandlingSettings,
};

pub struct TwoPlayer {
  game1: Game,
  game2: Game,
}

impl TwoPlayer {
  const CONTROLS1: Controller = Controller {
    restart: |_| false,
    swap: |rl| rl.is_key_pressed(KeyboardKey::KEY_E),
    left: |rl| rl.is_key_pressed(KeyboardKey::KEY_A),
    right: |rl| rl.is_key_pressed(KeyboardKey::KEY_D),
    left_das: |rl| rl.is_key_down(KeyboardKey::KEY_A),
    right_das: |rl| rl.is_key_down(KeyboardKey::KEY_D),
    clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_W),
    counter_clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_Q),
    one_eighty: |rl| rl.is_key_pressed(KeyboardKey::KEY_R),
    hard_drop: |rl| rl.is_key_pressed(KeyboardKey::KEY_Z),
    soft_drop: |rl| rl.is_key_down(KeyboardKey::KEY_S),
    undo: |_| false,
    pause: |rl| rl.is_key_pressed(KeyboardKey::KEY_ENTER),
    quit: |rl| rl.is_key_pressed(KeyboardKey::KEY_ESCAPE),
  };

  const CONTROLS2: Controller = Controller {
    restart: |_| false,
    swap: |rl| rl.is_key_pressed(KeyboardKey::KEY_O),
    left: |rl| rl.is_key_pressed(KeyboardKey::KEY_J),
    right: |rl| rl.is_key_pressed(KeyboardKey::KEY_L),
    left_das: |rl| rl.is_key_down(KeyboardKey::KEY_J),
    right_das: |rl| rl.is_key_down(KeyboardKey::KEY_L),
    clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_I),
    counter_clockwise: |rl| rl.is_key_pressed(KeyboardKey::KEY_U),
    one_eighty: |rl| rl.is_key_pressed(KeyboardKey::KEY_P),
    hard_drop: |rl| rl.is_key_pressed(KeyboardKey::KEY_M),
    soft_drop: |rl| rl.is_key_down(KeyboardKey::KEY_K),
    undo: |_| false,
    pause: |rl| rl.is_key_pressed(KeyboardKey::KEY_ENTER),
    quit: |rl| rl.is_key_pressed(KeyboardKey::KEY_ESCAPE),
  };

  fn drawing_details1(rl: &RaylibHandle) -> DrawingDetails {
    let block_length = DrawingDetails::HEIGHT_SCALE_FACTOR * 0.75 * rl.get_screen_height() as f32
      / playfield::VISIBLE_HEIGHT as f32;
    let position =
      (screen_vector(rl) * Vector2 { x: 0.5, y: 1.0 } - PLAYFIELD_VECTOR * block_length) / 2.0;
    DrawingDetails::new(block_length, position)
  }

  fn drawing_details2(rl: &RaylibHandle) -> DrawingDetails {
    let block_length = DrawingDetails::HEIGHT_SCALE_FACTOR * 0.75 * rl.get_screen_height() as f32
      / playfield::VISIBLE_HEIGHT as f32;
    let position =
      (screen_vector(rl) * Vector2 { x: 1.5, y: 1.0 } - PLAYFIELD_VECTOR * block_length) / 2.0;
    DrawingDetails::new(block_length, position)
  }

  pub fn new(settings1: HandlingSettings, settings2: HandlingSettings, rl: &RaylibHandle) -> Self {
    let game1 = Game::new(Self::drawing_details1(rl), Self::CONTROLS1, settings1);
    let game2 = Game::new(Self::drawing_details2(rl), Self::CONTROLS2, settings2);
    Self { game1, game2 }
  }

  pub fn update(&mut self, rl: &RaylibHandle) {
    self.game1.update(rl);
    self.game2.update(rl);
    self.game2.pause = self.game1.pause;
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    self.game1.draw(rld);
    self.game2.draw(rld);
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    (self.game1.controller.quit)(rl) && (self.game1.pause || self.game1.playfield.lost())
  }
}
