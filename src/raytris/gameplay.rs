mod game;
mod line_clear_message;
mod playfield;
pub mod single_player;
pub mod two_player;

use raylib::{RaylibHandle, color::Color, math::Vector2};
use serde::{Deserialize, Serialize};
use std::time::Duration;

type Input = fn(&RaylibHandle) -> bool;
struct Controller {
  restart: Input,
  swap: Input,
  left: Input,
  right: Input,
  left_das: Input,
  right_das: Input,
  clockwise: Input,
  counter_clockwise: Input,
  one_eighty: Input,
  hard_drop: Input,
  soft_drop: Input,
  undo: Input,
  pause: Input,
  quit: Input,
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
  const GRID_BACKGROUND_COLOR: Color = Color::BLACK;
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

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlingSettings {
  pub gravity: Duration,
  pub soft_drop: Duration,
  pub das: Duration,
  pub lock_delay: Duration,
  pub lock_delay_resets: u32,
}

impl Default for HandlingSettings {
  fn default() -> Self {
    Self {
      gravity: Duration::from_millis(330),
      soft_drop: Duration::from_millis(160),
      das: Duration::from_millis(140),
      lock_delay: Duration::from_millis(500),
      lock_delay_resets: 15,
    }
  }
}
