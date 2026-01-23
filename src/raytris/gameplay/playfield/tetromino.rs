use raylib::color::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tetromino {
  I,
  O,
  T,
  Z,
  S,
  J,
  L,
  Empty,
}

pub type TetrominoMap = [(i8, i8); 4];

impl Tetromino {
  pub fn color(self) -> Color {
    match self {
      Self::I => Color::new(49, 199, 239, 255),
      Self::O => Color::new(247, 211, 8, 255),
      Self::T => Color::new(173, 77, 156, 255),
      Self::Z => Color::new(239, 32, 41, 255),
      Self::S => Color::new(66, 182, 66, 255),
      Self::J => Color::new(90, 101, 173, 255),
      Self::L => Color::new(239, 121, 33, 255),
      Self::Empty => Color::BLANK,
    }
  }
  pub fn initial_map(self) -> TetrominoMap {
    match self {
      Tetromino::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
      Tetromino::O => [(0, -1), (1, -1), (0, 0), (1, 0)],
      Tetromino::T => [(0, -1), (-1, 0), (0, 0), (1, 0)],
      Tetromino::S => [(0, -1), (1, -1), (-1, 0), (0, 0)],
      Tetromino::Z => [(-1, -1), (0, -1), (0, 0), (1, 0)],
      Tetromino::J => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
      Tetromino::L => [(1, -1), (-1, 0), (0, 0), (1, 0)],
      Tetromino::Empty => [(0, 0), (0, 0), (0, 0), (0, 0)],
    }
  }
}
