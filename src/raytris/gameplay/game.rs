use std::time::Duration;

use raylib::{
  RaylibHandle,
  color::Color,
  math::Vector2,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails,
    line_clear_message::{LineClearMessage, MessageType, SpinType},
    playfield::{self, Playfield, UpdateInfo},
  },
  settings::handling::HandlingSettings,
};

#[derive(Clone)]
pub struct Game {
  playfield: Playfield,
  combo: u32,
  score: u64,
  b2b: u32,
  message: LineClearMessage,
}

impl Game {
  pub fn new() -> Self {
    Game {
      playfield: Playfield::new(),
      combo: 0,
      score: 0,
      b2b: 0,
      message: LineClearMessage::empty(),
    }
  }

  pub fn update(
    &mut self,
    dt: Duration,
    controller: &Controller,
    settings: &HandlingSettings,
    rl: &RaylibHandle,
  ) -> bool {
    self.message.remaining_time = self.message.remaining_time.saturating_sub(dt);

    if let Some(update_info) = self.playfield.update(controller, settings, dt, rl) {
      self.update_score(update_info);
      true
    } else {
      false
    }
  }

  pub fn reset(&mut self) {
    self.playfield = Playfield::new();
    self.combo = 0;
    self.b2b = 0;
    self.message = LineClearMessage::empty();
  }

  pub fn has_lost(&self) -> bool {
    self.playfield.has_lost()
  }

  fn update_score(&mut self, update_info: UpdateInfo) {
    let UpdateInfo {
      cleared_lines,
      spin,
      is_all_clear,
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
  }

  pub fn draw(&self, drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    self.playfield.draw(drawing_details, rld);

    if self.message.remaining_time > Duration::ZERO {
      self.draw_message(drawing_details, rld);
    }

    if self.combo >= 2 {
      self.draw_combo(drawing_details, rld);
    }

    if self.b2b >= 2 {
      self.draw_b2b(drawing_details, rld);
    }

    self.draw_score(drawing_details, rld);
  }

  fn draw_message(&self, drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    const MAX_DURATION: f32 = LineClearMessage::DURATION.as_secs_f32();
    let duration = self.message.remaining_time.as_secs_f32();
    let alpha = (255.0 * duration / MAX_DURATION) as u8;
    if let Some(message) = self.message.message {
      let Vector2 { x, y } = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 4,
        drawing_details,
      );
      let (msg, mut color) = message.info();
      color.a = alpha;
      rld.draw_text(msg, x as i32, y as i32, drawing_details.font_size, color);
    }

    if let Some((tetromino, spin_type)) = self.message.spin {
      let font_size = drawing_details.font_size;
      let font_size_small = drawing_details.font_size_small;
      let mut spin_color = tetromino.color();
      spin_color.a = alpha;
      let Vector2 { x, y } = get_block(
        DrawingDetails::LEFT_BORDER,
        playfield::HEIGHT - 6,
        drawing_details,
      );
      let spin_text = format!("{}-SPIN", tetromino.name());
      rld.draw_text(&spin_text, x as i32, y as i32, font_size, spin_color);
      if spin_type == SpinType::Mini {
        let Vector2 { x, y } = get_block(
          DrawingDetails::LEFT_BORDER,
          playfield::HEIGHT - 7,
          drawing_details,
        );
        rld.draw_text("MINI", x as i32, y as i32, font_size_small, spin_color);
      }
    }
  }

  fn draw_combo(&self, drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    const COMBO_TEXT: &str = "COMBO ";
    let font_size = drawing_details.font_size;
    let Vector2 { x, y } = get_block(
      DrawingDetails::LEFT_BORDER,
      playfield::HEIGHT - 10,
      drawing_details,
    );
    let (x, y) = (x as i32, y as i32);
    let combo = format!("{}", self.combo);
    let x_offset = rld.measure_text(COMBO_TEXT, drawing_details.font_size);

    rld.draw_text(COMBO_TEXT, x, y, font_size, Color::BLUE);
    rld.draw_text(&combo, x + x_offset, y, font_size, Color::BLUE);
  }

  fn draw_b2b(&self, drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    const B2B_TEXT: &str = "B2B ";
    let font_size = drawing_details.font_size;
    let Vector2 { x, y } = get_block(
      DrawingDetails::LEFT_BORDER,
      playfield::HEIGHT - 12,
      drawing_details,
    );
    let (x, y) = (x as i32, y as i32);
    let b2b = format!("{}", self.b2b - 1);
    let x_offset = rld.measure_text(B2B_TEXT, drawing_details.font_size);

    rld.draw_text(B2B_TEXT, x, y, font_size, Color::BLUE);
    rld.draw_text(&b2b, x + x_offset, y, font_size, Color::BLUE);
  }

  fn draw_score(&self, drawing_details: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let font_size = drawing_details.font_size;
    let Vector2 { x, y } = get_block(playfield::WIDTH + 1, playfield::HEIGHT - 2, drawing_details);
    let (x, y) = (x as i32, y as i32);
    let score = format!("{:09}", self.score);
    let y_offset = (drawing_details.block_length / 2.0) as i32;

    rld.draw_text(
      &score,
      x,
      y + y_offset,
      font_size,
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

pub fn screen_vector(rl: &RaylibHandle) -> Vector2 {
  Vector2 {
    x: rl.get_screen_width() as f32,
    y: rl.get_screen_height() as f32,
  }
}

pub const PLAYFIELD_VECTOR: Vector2 = Vector2 {
  x: playfield::WIDTH as f32,
  y: playfield::VISIBLE_HEIGHT as f32,
};
