use std::time::Duration;

use raylib::color::Color;

use crate::raytris::gameplay::playfield::tetromino::Tetromino;

#[derive(Clone, Copy)]
pub enum MessageType {
  Single,
  Double,
  Triple,
  Tetris,
  AllClear,
}

impl MessageType {
  pub fn info(self) -> (&'static str, Color) {
    match self {
      Self::Single => ("SINGLE", Color::new(0, 0, 0, 255)),
      Self::Double => ("DOUBLE", Color::new(235, 149, 52, 255)),
      Self::Triple => ("TRIPLE", Color::new(88, 235, 52, 255)),
      Self::Tetris => ("TETRIS", Color::new(52, 164, 236, 255)),
      Self::AllClear => ("ALL\nCLEAR", Color::new(235, 52, 213, 255)),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpinType {
  Mini,
  Proper,
}

#[derive(Clone)]
pub struct LineClearMessage {
  pub message: Option<MessageType>,
  pub remaining_time: Duration,
  pub spin: Option<(Tetromino, SpinType)>,
}

impl LineClearMessage {
  pub const DURATION: Duration = Duration::from_secs(2);

  pub fn empty() -> Self {
    Self {
      message: None,
      remaining_time: Duration::ZERO,
      spin: None,
    }
  }

  pub fn new(message: Option<MessageType>, spin: Option<(Tetromino, SpinType)>) -> Self {
    Self {
      message,
      remaining_time: Self::DURATION,
      spin,
    }
  }
}
