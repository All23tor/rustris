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
    playfield::{Playfield, UpdateInfo, tetromino::Tetromino},
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
  const GRINDLINE_COLOR: Color = Color::DARKGRAY;
  const UNAVAILABLE_HOLD_PIECE_COLOR: Color = Color::DARKGRAY;
  const PIECES_BACKGROUND_COLOR: Color = Color::GRAY;
  const INFO_TEXT_COLOR: Color = Color::BLACK;
  const PIECE_BOX_COLOR: Color = Color::BLACK;
  const YOU_LOST_COLOR: Color = Color::RED;
  const GAME_PAUSED_COLOR: Color = Color::BLUE;
  const QUIT_COLOR: Color = Color::WHITE;
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
      self.playfield = Playfield::new();
      self.combo = 0;
      self.b2b = 0;
      self.message = LineClearMessage::empty();
    }

    if (self.controller.pause)(rl) {
      self.pause = !self.pause;
    }
    if self.pause || self.has_lost {
      false
    } else {
      self.message.remaining_time = self.message.remaining_time.saturating_sub(dt);
      let update_info = self
        .playfield
        .update(&self.controller, &self.settings, dt, rl);

      let Some(UpdateInfo {
        cleared_lines,
        spin_type,
        is_all_clear,
        has_lost,
      }) = update_info
      else {
        return false;
      };

      if cleared_lines == 0 {
        self.combo = 0;
      } else {
        self.combo += 1;
        if cleared_lines == 4 || spin_type.is_some() {
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
      let spin_index = match spin_type {
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

      self.message = LineClearMessage::new(message, spin_type);

      if is_all_clear {
        self.message.message = Some(MessageType::AllClear);
        self.score += 3500 * b2b_factor / 2;
      }

      self.has_lost = has_lost;
      true
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    self.playfield.draw(&self.drawing_details, rld);
    self.draw_info(rld);

    if !self.has_lost && !self.pause {
      return;
    }

    let (width, height) = (rld.get_screen_width(), rld.get_render_height());
    rld.draw_rectangle(0, 0, width, height, DrawingDetails::DARKEN_COLOR);

    if self.has_lost {
      rld.draw_text(
        "YOU LOST",
        (width - rld.measure_text("YOU LOST", self.drawing_details.font_size_big)) / 2,
        height / 2,
        self.drawing_details.font_size_big,
        DrawingDetails::YOU_LOST_COLOR,
      );
    } else if self.pause {
      rld.draw_text(
        "GAME PAUSED",
        (width - rld.measure_text("GAME PAUSED", self.drawing_details.font_size_big)) / 2,
        height / 2,
        self.drawing_details.font_size_big,
        DrawingDetails::GAME_PAUSED_COLOR,
      );
    }
    rld.draw_text(
      "Press Esc to quit",
      (width - rld.measure_text("Press Enter to quit", self.drawing_details.font_size)) / 2,
      height / 2 + self.drawing_details.font_size_big,
      self.drawing_details.font_size,
      DrawingDetails::QUIT_COLOR,
    );
  }

  fn draw_info(&self, rld: &mut RaylibDrawHandle) {
    if self.message.remaining_time > Duration::ZERO {
      let alpha = 255.0 * self.message.remaining_time.as_secs_f32()
        / LineClearMessage::DURATION.as_secs_f32();
      if let Some(message) = self.message.message {
        let text_rect = get_block(
          DrawingDetails::LEFT_BORDER,
          playfield::HEIGHT - 4,
          &self.drawing_details,
        );
        let (msg, mut color) = message.info();
        color.a = alpha as u8;
        rld.draw_text(
          msg,
          text_rect.x as i32,
          text_rect.y as i32,
          self.drawing_details.font_size,
          color,
        );
      }

      if let Some(spin_type) = self.message.spin_type {
        let mut spin_color = Tetromino::T.color();
        spin_color.a = alpha as u8;
        let spin_rect = get_block(
          DrawingDetails::LEFT_BORDER,
          playfield::HEIGHT - 6,
          &self.drawing_details,
        );
        rld.draw_text(
          "TSPIN",
          spin_rect.x as i32,
          spin_rect.y as i32,
          self.drawing_details.font_size,
          spin_color,
        );
        if spin_type == SpinType::Mini {
          let mini_rect = get_block(
            DrawingDetails::LEFT_BORDER,
            playfield::HEIGHT - 7,
            &self.drawing_details,
          );
          rld.draw_text(
            "MINI",
            mini_rect.x as i32,
            mini_rect.y as i32,
            self.drawing_details.font_size_small,
            spin_color,
          );
        }
      }
    }

    if self.combo >= 2 {
      let combo_rect = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 10,
        &self.drawing_details,
      );
      rld.draw_text(
        "COMBO ",
        combo_rect.x as i32,
        combo_rect.y as i32,
        self.drawing_details.font_size,
        Color::BLUE,
      );
      rld.draw_text(
        &format!("{}", self.combo),
        combo_rect.x as i32 + rld.measure_text("COMBO ", self.drawing_details.font_size),
        combo_rect.y as i32,
        self.drawing_details.font_size,
        Color::BLUE,
      );
    }

    if self.b2b >= 2 {
      let b2b_rect = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 12,
        &self.drawing_details,
      );
      rld.draw_text(
        "B2B ",
        b2b_rect.x as i32,
        b2b_rect.y as i32,
        self.drawing_details.font_size,
        Color::BLUE,
      );
      rld.draw_text(
        &format!("{}", self.b2b - 1),
        b2b_rect.x as i32 + rld.measure_text("B2B ", self.drawing_details.font_size),
        b2b_rect.y as i32,
        self.drawing_details.font_size,
        Color::BLUE,
      );
    }

    let score_rect = get_block(
      playfield::WIDTH + 1,
      playfield::HEIGHT - 2,
      &self.drawing_details,
    );
    rld.draw_text(
      &format!("{:09}", self.score),
      score_rect.x as i32,
      (score_rect.y + self.drawing_details.block_length * 0.5) as i32,
      self.drawing_details.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
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
