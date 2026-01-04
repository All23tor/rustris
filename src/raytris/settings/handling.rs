use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlingSettings {
  pub gravity: i32,
  pub soft_drop: i32,
  pub lock_delay_frames: i32,
  pub lock_delay_resets: i32,
  pub das: i32,
}

impl Default for HandlingSettings {
  fn default() -> Self {
    Self {
      gravity: 20,
      soft_drop: 1,
      lock_delay_frames: 30,
      lock_delay_resets: 15,
      das: 7,
    }
  }
}
