use rand::{rng, seq::SliceRandom};

use crate::raytris::gameplay::playfield::tetromino::Tetromino;

pub const NEXT_SIZE: usize = 5;
pub const SIZE_OF_BAG: usize = 7;
pub const MAX_QUEUE_SIZE: usize = NEXT_SIZE + SIZE_OF_BAG;

#[derive(Clone)]
pub struct NextQueue {
  queue: [Tetromino; MAX_QUEUE_SIZE],
  size: usize,
}

impl NextQueue {
  const NEW_BAG: [Tetromino; SIZE_OF_BAG] = [
    Tetromino::I,
    Tetromino::O,
    Tetromino::T,
    Tetromino::S,
    Tetromino::Z,
    Tetromino::J,
    Tetromino::L,
  ];

  pub fn new() -> Self {
    let mut queue = [Tetromino::Empty; MAX_QUEUE_SIZE];
    queue[0..SIZE_OF_BAG].copy_from_slice(&Self::NEW_BAG);
    queue[0..SIZE_OF_BAG].shuffle(&mut rng());

    Self {
      queue,
      size: SIZE_OF_BAG,
    }
  }

  pub fn next_tetromino(&mut self) -> Tetromino {
    self.size -= 1;
    let next = self.queue[self.size];
    if self.size <= NEXT_SIZE {
      self.queue.copy_within(0..self.size, SIZE_OF_BAG);
      self.queue[0..SIZE_OF_BAG].copy_from_slice(&Self::NEW_BAG);
      self.queue[0..SIZE_OF_BAG].shuffle(&mut rng());
      self.size += SIZE_OF_BAG;
    }
    next
  }

  pub fn peek(&self) -> Tetromino {
    self.queue[self.size - 1]
  }

  pub fn queue(&self) -> impl Iterator<Item = Tetromino> {
    self.queue.iter().take(self.size).rev().cloned()
  }
}
