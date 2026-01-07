use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlingSettings {
  pub gravity: u32,
  pub soft_drop: u32,
  pub lock_delay_frames: u32,
  pub lock_delay_resets: u32,
  pub das: u32,
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
