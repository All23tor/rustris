use rand::{rng, seq::SliceRandom};
use std::collections::VecDeque;

use crate::raytris::gameplay::playfield::tetromino::Tetromino;

pub const NEXT_SIZE: usize = 5;
const SIZE_OF_BAG: usize = 7;
const MAX_QUEUE_SIZE: usize = NEXT_SIZE + SIZE_OF_BAG;

#[derive(Clone)]
pub struct NextQueue {
  queue: VecDeque<Tetromino>,
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
    let mut queue = VecDeque::with_capacity(MAX_QUEUE_SIZE);
    Self::push_new_bag(&mut queue);
    Self { queue }
  }

  fn push_new_bag(queue: &mut VecDeque<Tetromino>) {
    let mut bag = Self::NEW_BAG;
    bag.shuffle(&mut rng());
    queue.extend(bag);
  }

  pub fn next_tetromino(&mut self) -> Tetromino {
    let tetromino = self.queue.pop_front().expect("queue should never be empty");
    if self.queue.len() < NEXT_SIZE {
      Self::push_new_bag(&mut self.queue);
    }

    tetromino
  }

  pub fn peek(&self) -> Tetromino {
    *self.queue.front().expect("queue should never be empty")
  }

  pub fn queue(&self) -> impl Iterator<Item = Tetromino> {
    self.queue.iter().take(NEXT_SIZE).copied()
  }
}
