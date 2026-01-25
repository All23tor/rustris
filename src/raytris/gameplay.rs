mod line_clear_message;
mod playfield;
pub mod single_player;
pub mod two_player;

use std::time::Duration;

use raylib::{
  RaylibHandle,
  color::Color,
  math::Vector2,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    line_clear_message::{LineClearMessage, MessageType, SpinType},
    playfield::{Playfield, UpdateInfo},
  },
  settings::handling::HandlingSettings,
};

type Input = fn(&RaylibHandle) -> bool;
struct Controller {
  pub restart: Input,
  pub swap: Input,
  pub left: Input,
  pub right: Input,
  pub left_das: Input,
  pub right_das: Input,
  pub clockwise: Input,
  pub counter_clockwise: Input,
  pub one_eighty: Input,
  pub hard_drop: Input,
  pub soft_drop: Input,
  pub undo: Input,
  pub pause: Input,
  pub quit: Input,
}

struct DrawingDetails {
  block_length: f32,
  position: Vector2,
  font_size: i32,
  font_size_big: i32,
  font_size_small: i32,
}

pub const BACKGROUND_COLOR: Color = DrawingDetails::BACKGROUND_COLOR;

impl DrawingDetails {
  const HEIGHT_SCALE_FACTOR: f32 = 0.80;
  const DEFAULT_PRETTY_OUTLINE: Color = Color::new(0, 0, 0, 255 / 8);
  const TETRION_BACKGROUND_COLOR: Color = Color::BLACK;
  const GRIDLINE_COLOR: Color = Color::DARKGRAY;
  const UNAVAILABLE_HOLD_PIECE_COLOR: Color = Color::DARKGRAY;
  const PIECES_BACKGROUND_COLOR: Color = Color::GRAY;
  const INFO_TEXT_COLOR: Color = Color::BLACK;
  const PIECE_BOX_COLOR: Color = Color::BLACK;
  const DARKEN_COLOR: Color = Color::new(0, 0, 0, 100);
  const BACKGROUND_COLOR: Color = Color::LIGHTGRAY;
  const LEFT_BORDER: i32 = -10;

  fn new(block_length: f32, position: Vector2) -> Self {
    Self {
      block_length,
      font_size: block_length as i32 * 2,
      font_size_big: block_length as i32 * 5,
      font_size_small: block_length as i32,
      position,
    }
  }
}

struct Game {
  pub drawing_details: DrawingDetails,
  pub controller: Controller,
  pub settings: HandlingSettings,
  pub playfield: Playfield,
  pub pause: bool,
  has_lost: bool,
  combo: u32,
  score: u64,
  b2b: u32,
  message: LineClearMessage,
}

impl Game {
  pub fn new(
    drawing_details: DrawingDetails,
    controller: Controller,
    settings: HandlingSettings,
  ) -> Self {
    Game {
      drawing_details,
      controller,
      settings,
      playfield: Playfield::new(),
      pause: false,
      has_lost: false,
      combo: 0,
      score: 0,
      b2b: 0,
      message: LineClearMessage::empty(),
    }
  }

  pub fn update(&mut self, dt: Duration, rl: &RaylibHandle) -> bool {
    if (self.controller.restart)(rl) {
      self.reset();
    }

    if (self.controller.pause)(rl) {
      self.pause = !self.pause;
    }
    if self.pause || self.has_lost {
      false
    } else {
      self.message.remaining_time = self.message.remaining_time.saturating_sub(dt);

      if let Some(update_info) = self
        .playfield
        .update(&self.controller, &self.settings, dt, rl)
      {
        self.update_score(update_info);
        true
      } else {
        false
      }
    }
  }

  fn reset(&mut self) {
    self.playfield = Playfield::new();
    self.has_lost = false;
    self.combo = 0;
    self.b2b = 0;
    self.message = LineClearMessage::empty();
  }

  fn update_score(&mut self, update_info: UpdateInfo) {
    let UpdateInfo {
      cleared_lines,
      spin,
      is_all_clear,
      has_lost,
    } = update_info;

    if cleared_lines == 0 {
      self.combo = 0;
    } else {
      self.combo += 1;
      if cleared_lines == 4 || spin.is_some() {
        self.b2b += 1;
      } else {
        self.b2b = 0;
      }
    }

    self.score += (self.combo * 50) as u64;
    let b2b_factor = if self.b2b >= 2 { 3 } else { 2 };
    const SCORE_TABLE: [[u64; 5]; 3] = [
      /* cleared:  0   1    2    3    4  */
      /*NoSpin */ [0, 100, 300, 500, 800],
      /*Mini   */ [100, 200, 400, 0, 0],
      /*Proper */ [400, 800, 1200, 1600, 0],
    ];
    let spin_index = match spin.map(|(_, spin_type)| spin_type) {
      None => 0,
      Some(SpinType::Mini) => 1,
      Some(SpinType::Proper) => 2,
    };
    self.score += b2b_factor * SCORE_TABLE[spin_index][cleared_lines as usize] / 2;

    let message = match cleared_lines {
      0 => None,
      1 => Some(MessageType::Single),
      2 => Some(MessageType::Double),
      3 => Some(MessageType::Triple),
      4 => Some(MessageType::Tetris),
      _ => panic!(),
    };

    self.message = LineClearMessage::new(message, spin);

    if is_all_clear {
      self.message.message = Some(MessageType::AllClear);
      self.score += 3500 * b2b_factor / 2;
    }

    self.has_lost = has_lost;
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    self.playfield.draw(&self.drawing_details, rld);

    if self.message.remaining_time > Duration::ZERO {
      self.draw_message(rld);
    }

    if self.combo >= 2 {
      self.draw_combo(rld);
    }

    if self.b2b >= 2 {
      self.draw_b2b(rld);
    }

    self.draw_score(rld);

    if self.has_lost {
      self.draw_lost(rld);
    } else if self.pause {
      self.draw_pause(rld);
    }
  }

  fn draw_message(&self, rld: &mut RaylibDrawHandle) {
    const MAX_DURATION: f32 = LineClearMessage::DURATION.as_secs_f32();
    let duration = self.message.remaining_time.as_secs_f32();
    let alpha = (255.0 * duration / MAX_DURATION) as u8;
    if let Some(message) = self.message.message {
      let Vector2 { x, y } = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 4,
        &self.drawing_details,
      );
      let (msg, mut color) = message.info();
      color.a = alpha;
      rld.draw_text(
        msg,
        x as i32,
        y as i32,
        self.drawing_details.font_size,
        color,
      );
    }

    if let Some((tetromino, spin_type)) = self.message.spin {
      let font_size = self.drawing_details.font_size;
      let font_size_small = self.drawing_details.font_size_small;
      let mut spin_color = tetromino.color();
      spin_color.a = alpha;
      let Vector2 { x, y } = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 6,
        &self.drawing_details,
      );
      let spin_text = format!("{}-SPIN", tetromino.name());
      rld.draw_text(&spin_text, x as i32, y as i32, font_size, spin_color);
      if spin_type == SpinType::Mini {
        let Vector2 { x, y } = get_block(
          DrawingDetails::LEFT_BORDER,
          playfield::HEIGHT - 7,
          &self.drawing_details,
        );
        rld.draw_text("MINI", x as i32, y as i32, font_size_small, spin_color);
      }
    }
  }

  fn draw_combo(&self, rld: &mut RaylibDrawHandle) {
    const COMBO_TEXT: &str = "COMBO ";
    let font_size = self.drawing_details.font_size;
    let Vector2 { x, y } = get_block(
      DrawingDetails::LEFT_BORDER,
      playfield::HEIGHT - 10,
      &self.drawing_details,
    );
    let (x, y) = (x as i32, y as i32);
    let combo = format!("{}", self.combo);
    let x_offset = rld.measure_text(COMBO_TEXT, self.drawing_details.font_size);

    rld.draw_text(COMBO_TEXT, x, y, font_size, Color::BLUE);
    rld.draw_text(&combo, x + x_offset, y, font_size, Color::BLUE);
  }

  fn draw_b2b(&self, rld: &mut RaylibDrawHandle) {
    const B2B_TEXT: &str = "B2B ";
    let font_size = self.drawing_details.font_size;
    let Vector2 { x, y } = get_block(
      DrawingDetails::LEFT_BORDER,
      playfield::HEIGHT - 12,
      &self.drawing_details,
    );
    let (x, y) = (x as i32, y as i32);
    let b2b = format!("{}", self.b2b - 1);
    let x_offset = rld.measure_text(B2B_TEXT, self.drawing_details.font_size);

    rld.draw_text(B2B_TEXT, x, y, font_size, Color::BLUE);
    rld.draw_text(&b2b, x + x_offset, y, font_size, Color::BLUE);
  }

  fn draw_score(&self, rld: &mut RaylibDrawHandle) {
    let font_size = self.drawing_details.font_size;
    let Vector2 { x, y } = get_block(
      playfield::WIDTH + 1,
      playfield::HEIGHT - 2,
      &self.drawing_details,
    );
    let (x, y) = (x as i32, y as i32);
    let score = format!("{:09}", self.score);
    let y_offset = (self.drawing_details.block_length / 2.0) as i32;

    rld.draw_text(
      &score,
      x,
      y + y_offset,
      font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );
  }

  fn draw_lost(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    let (half_width, half_height) = (width / 2, height / 2);
    let font_size_big = self.drawing_details.font_size_big;

    const LOST_COLOR: Color = Color::RED;
    const LOST_TEXT: &str = "YOU LOST";
    let x_offset = -rld.measure_text(LOST_TEXT, self.drawing_details.font_size_big) / 2;

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
    let x_offset = -rld.measure_text(PAUSED_TEXT, self.drawing_details.font_size_big) / 2;

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
    let x_offset = -rld.measure_text(QUIT_TEXT, self.drawing_details.font_size) / 2;
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

fn get_block(i: i32, j: i32, d: &DrawingDetails) -> Vector2 {
  Vector2 {
    x: d.position.x + i as f32 * d.block_length,
    y: d.position.y + (j - playfield::VISIBLE_HEIGHT) as f32 * d.block_length,
  }
}

fn screen_vector(rl: &RaylibHandle) -> Vector2 {
  Vector2 {
    x: rl.get_screen_width() as f32,
    y: rl.get_screen_height() as f32,
  }
}

const PLAYFIELD_VECTOR: Vector2 = Vector2 {
  x: playfield::WIDTH as f32,
  y: playfield::VISIBLE_HEIGHT as f32,
};
