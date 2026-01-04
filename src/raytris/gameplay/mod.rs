use raylib::{
  RaylibHandle,
  color::Color,
  math::Vector2,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{gameplay::playfield::Playfield, settings::handling::HandlingSettings};

mod playfield;
pub mod single_player;
pub mod two_player;

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
    }
  }

  pub fn update(&mut self, rl: &RaylibHandle) -> bool {
    if (self.controller.restart)(rl) {
      self.playfield.restart();
    }
    if (self.controller.pause)(rl) {
      self.pause = !self.pause;
    }
    if self.pause {
      false
    } else {
      self.playfield.update(&self.controller, &self.settings, rl)
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    self.playfield.draw(&self.drawing_details, rld);

    if !self.playfield.lost() && !self.pause {
      return;
    }

    let width = rld.get_screen_width();
    let height = rld.get_render_height();
    rld.draw_rectangle(0, 0, width, height, DrawingDetails::DARKEN_COLOR);

    if self.playfield.lost() {
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
}

fn screen_vector(rl: &RaylibHandle) -> Vector2 {
  Vector2 {
    x: rl.get_screen_width() as f32,
    y: rl.get_screen_height() as f32,
  }
}

const PLAYFIELD_VECTOR: Vector2 = Vector2 {
  x: Playfield::WIDTH as f32,
  y: Playfield::VISIBLE_HEIGHT as f32,
};
