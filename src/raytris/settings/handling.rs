use std::time::Duration;

use serde::{Deserialize, Serialize};

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
