use std::time::Duration;

use raylib::{
  RaylibHandle,
  color::Color,
  consts::KeyboardKey,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails,
    game::{Game, PLAYFIELD_VECTOR, screen_vector},
    playfield::{self},
  },
  settings::handling::HandlingSettings,
};

pub struct SinglePlayer {
  game: Game,
  pause: bool,
  drawing_details: DrawingDetails,
  handling_settings: HandlingSettings,
  undo_stack: Vec<Game>,
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
    soft_drop: |rl| rl.is_key_down(KeyboardKey::KEY_DOWN),
    undo: |rl| {
      rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_Z)
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

  pub fn new(handling_settings: HandlingSettings, rl: &RaylibHandle) -> Self {
    let game = Game::new();
    let pause = false;
    let drawing_details = Self::drawing_details(rl);
    let undo_stack = vec![game.clone()];
    Self {
      game,
      pause,
      drawing_details,
      handling_settings,
      undo_stack,
    }
  }
  pub fn update(&mut self, dt: Duration, rl: &RaylibHandle) {
    if (Self::KEYBOARD_CONTROLS.undo)(rl)
      && let Some(top) = self.undo_stack.pop()
    {
      self.game = top;
      if self.undo_stack.is_empty() {
        self.undo_stack.push(self.game.clone());
      }
      return;
    }

    if (Self::KEYBOARD_CONTROLS.restart)(rl) {
      self.game.reset();
    }

    if (Self::KEYBOARD_CONTROLS.pause)(rl) {
      self.pause = !self.pause;
    }

    if !self.pause
      && self
        .game
        .update(dt, &Self::KEYBOARD_CONTROLS, &self.handling_settings, rl)
    {
      self.undo_stack.push(self.game.clone());
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    rld.clear_background(DrawingDetails::BACKGROUND_COLOR);
    self.game.draw(&self.drawing_details, rld);

    if self.pause {
      self.draw_pause(rld);
    } else if self.game.has_lost() {
      self.draw_lost(rld);
    }
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    (Self::KEYBOARD_CONTROLS.quit)(rl) && (self.pause || self.game.has_lost())
  }

  fn draw_lost(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (half_width, half_height) = (width / 2, height / 2);
    let font_size_big = self.drawing_details.font_size_big;

    const LOST_COLOR: Color = Color::RED;
    const LOST_TEXT: &str = "YOU LOST";
    let x_offset = -rld.measure_text(LOST_TEXT, font_size_big) / 2;

    rld.draw_rectangle(0, 0, width, height, DrawingDetails::DARKEN_COLOR);
    rld.draw_text(
      LOST_TEXT,
      half_width + x_offset,
      half_height,
      font_size_big,
      LOST_COLOR,
    );

    self.draw_quit(rld);
  }

  fn draw_pause(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (half_width, half_height) = (width / 2, height / 2);
    let font_size_big = self.drawing_details.font_size_big;

    const PAUSED_COLOR: Color = Color::BLUE;
    const PAUSED_TEXT: &str = "GAME PAUSED";
    let x_offset = -rld.measure_text(PAUSED_TEXT, font_size_big) / 2;

    rld.draw_rectangle(0, 0, width, height, DrawingDetails::DARKEN_COLOR);
    rld.draw_text(
      PAUSED_TEXT,
      half_width + x_offset,
      half_height,
      font_size_big,
      PAUSED_COLOR,
    );

    self.draw_quit(rld);
  }

  fn draw_quit(&self, rld: &mut RaylibDrawHandle) {
    let font_size = self.drawing_details.font_size;
    let font_size_big = self.drawing_details.font_size_big;
    let (half_width, half_height) = (rld.get_screen_width() / 2, rld.get_render_height() / 2);

    const QUIT_COLOR: Color = Color::WHITE;
    const QUIT_TEXT: &str = "Press Esc to quit";
    let x_offset = -rld.measure_text(QUIT_TEXT, font_size) / 2;
    let y_offset = font_size_big;

    rld.draw_text(
      QUIT_TEXT,
      half_width + x_offset,
      half_height + y_offset,
      font_size,
      QUIT_COLOR,
    );
  }
}
