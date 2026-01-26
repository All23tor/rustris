use std::time::Duration;

use raylib::{
  RaylibHandle,
  color::Color,
  consts::KeyboardKey,
  math::Vector2,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails,
    game::{Game, PLAYFIELD_VECTOR, screen_vector},
    playfield,
  },
  settings::handling::HandlingSettings,
};

pub struct TwoPlayer {
  games: [(Game, DrawingDetails, HandlingSettings); 2],
  pause: bool,
}

impl TwoPlayer {
  const CONTROLS0: Controller = Controller {
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

  const CONTROLS1: Controller = Controller {
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

  fn drawing_details0(rl: &RaylibHandle) -> DrawingDetails {
    let block_length = DrawingDetails::HEIGHT_SCALE_FACTOR * 0.75 * rl.get_screen_height() as f32
      / playfield::VISIBLE_HEIGHT as f32;
    let position =
      (screen_vector(rl) * Vector2 { x: 0.5, y: 1.0 } - PLAYFIELD_VECTOR * block_length) / 2.0;
    DrawingDetails::new(block_length, position)
  }

  fn drawing_details1(rl: &RaylibHandle) -> DrawingDetails {
    let block_length = DrawingDetails::HEIGHT_SCALE_FACTOR * 0.75 * rl.get_screen_height() as f32
      / playfield::VISIBLE_HEIGHT as f32;
    let position =
      (screen_vector(rl) * Vector2 { x: 1.5, y: 1.0 } - PLAYFIELD_VECTOR * block_length) / 2.0;
    DrawingDetails::new(block_length, position)
  }

  pub fn new(settings1: HandlingSettings, settings2: HandlingSettings, rl: &RaylibHandle) -> Self {
    Self {
      games: [
        (Game::new(), Self::drawing_details0(rl), settings1),
        (Game::new(), Self::drawing_details1(rl), settings2),
      ],
      pause: false,
    }
  }

  pub fn update(&mut self, dt: Duration, rl: &RaylibHandle) {
    if (Self::CONTROLS0.pause)(rl) {
      self.pause = !self.pause;
    }

    if !self.pause {
      let [(game0, _, hand_set0), (game1, _, hand_set1)] = &mut self.games;
      game0.update(dt, &Self::CONTROLS0, hand_set0, rl);
      game1.update(dt, &Self::CONTROLS1, hand_set1, rl);
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    rld.clear_background(DrawingDetails::BACKGROUND_COLOR);
    let [(game0, drdet0, _), (game1, drdet1, _)] = &self.games;
    game0.draw(drdet0, rld);
    game1.draw(drdet1, rld);

    if self.pause {
      Self::draw_pause(drdet0, rld);
    } else {
      if game0.has_lost() {
        self.draw_lost0(rld);
      }
      if game1.has_lost() {
        self.draw_lost1(rld);
      }
    }
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    let [(game0, _, _), (game1, _, _)] = &self.games;
    let has_lost = game0.has_lost() && game1.has_lost();
    (Self::CONTROLS0.quit)(rl) && (self.pause || has_lost)
  }

  fn draw_pause(drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (half_width, half_height) = (width / 2, height / 2);
    let font_size = drawing_details.font_size;
    let font_size_big = drawing_details.font_size_big;

    const PAUSED_COLOR: Color = Color::BLUE;
    const PAUSED_TEXT: &str = "GAME PAUSED";
    let x_offset = -rld.measure_text(PAUSED_TEXT, drawing_details.font_size_big) / 2;

    rld.draw_rectangle(0, 0, width, height, DrawingDetails::DARKEN_COLOR);
    rld.draw_text(
      PAUSED_TEXT,
      half_width + x_offset,
      half_height,
      font_size_big,
      PAUSED_COLOR,
    );

    const QUIT_COLOR: Color = Color::WHITE;
    const QUIT_TEXT: &str = "Press Esc to quit";
    let x_offset = -rld.measure_text(QUIT_TEXT, drawing_details.font_size) / 2;
    let y_offset = font_size_big;

    rld.draw_text(
      QUIT_TEXT,
      half_width + x_offset,
      half_height + y_offset,
      font_size,
      QUIT_COLOR,
    );
  }

  fn draw_lost0(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (_, drawing_details, _) = &self.games[0];
    let font_size_big = drawing_details.font_size_big;

    const LOST_COLOR: Color = Color::RED;
    const LOST_TEXT: &str = "YOU LOST";
    let x_offset = -rld.measure_text(LOST_TEXT, font_size_big) / 2;

    rld.draw_rectangle(0, 0, width / 2, height, DrawingDetails::DARKEN_COLOR);
    rld.draw_text(
      LOST_TEXT,
      width / 4 + x_offset,
      height / 2,
      font_size_big,
      LOST_COLOR,
    );
  }

  fn draw_lost1(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (_, drawing_details, _) = &self.games[1];
    let font_size_big = drawing_details.font_size_big;

    const LOST_COLOR: Color = Color::RED;
    const LOST_TEXT: &str = "YOU LOST";
    let x_offset = -rld.measure_text(LOST_TEXT, font_size_big) / 2;

    rld.draw_rectangle(
      width / 2,
      0,
      width / 2,
      height,
      DrawingDetails::DARKEN_COLOR,
    );
    rld.draw_text(
      LOST_TEXT,
      3 * width / 4 + x_offset,
      height / 2,
      font_size_big,
      LOST_COLOR,
    );
  }
}
