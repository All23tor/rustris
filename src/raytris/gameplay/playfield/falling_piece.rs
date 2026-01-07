use crate::raytris::gameplay::playfield::tetromino::{Tetromino, TetrominoMap};

#[derive(Clone, Copy)]
enum Orientation {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Clone, Copy)]
pub enum Shift {
  Left,
  Right,
}

#[derive(Clone, Copy)]
enum RotationType {
  Clockwise,
  CounterClockwise,
  OneEighty,
}

#[derive(Clone)]
pub struct FallingPiece {
  pub tetromino: Tetromino,
  pub orientation: Orientation,
  pub x: i8,
  pub y: i8,
  pub map: TetrominoMap,
}

impl FallingPiece {
  pub fn new(tetromino: Tetromino, x: i8, y: i8) -> Self {
    Self {
      tetromino,
      orientation: Orientation::Up,
      x,
      y,
      map: tetromino.initial_map(),
    }
  }

  pub fn fall(&mut self) {
    self.y += 1;
  }

  pub fn unfall(&mut self) {
    self.y -= 1;
  }

  pub fn shift(&mut self, shift: Shift) {
    self.x += match shift {
      Shift::Left => -1,
      Shift::Right => 1,
    }
  }

  pub fn rotate(&mut self, rt: RotationType) {
    for coord in &mut self.map.0 {
      *coord = match rt {
        RotationType::Clockwise => (-coord.1, coord.0),
        RotationType::CounterClockwise => (coord.1, coord.0),
        RotationType::OneEighty => (-coord.0, -coord.1),
      }
    }

    self.orientation = match (self.orientation, rt) {
      (Orientation::Up, RotationType::Clockwise) => Orientation::Right,
      (Orientation::Right, RotationType::Clockwise) => Orientation::Down,
      (Orientation::Down, RotationType::Clockwise) => Orientation::Left,
      (Orientation::Left, RotationType::Clockwise) => Orientation::Up,

      (Orientation::Up, RotationType::CounterClockwise) => Orientation::Left,
      (Orientation::Left, RotationType::CounterClockwise) => Orientation::Down,
      (Orientation::Down, RotationType::CounterClockwise) => Orientation::Right,
      (Orientation::Right, RotationType::CounterClockwise) => Orientation::Up,

      (Orientation::Up, RotationType::OneEighty) => Orientation::Down,
      (Orientation::Right, RotationType::OneEighty) => Orientation::Left,
      (Orientation::Down, RotationType::OneEighty) => Orientation::Up,
      (Orientation::Left, RotationType::OneEighty) => Orientation::Right,
    }
  }

  pub fn translate(&mut self, translation: (i8, i8)) {
    self.x += translation.0;
    self.y += translation.1;
  }
}
