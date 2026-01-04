use raylib::window::{get_current_monitor, get_monitor_height, get_monitor_width};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Resolution {
  Small,
  #[default]
  Medium,
  Big,
  Fullscreen,
}

impl Resolution {
  pub fn next(self) -> Self {
    match self {
      Resolution::Small => Resolution::Medium,
      Resolution::Medium => Self::Big,
      Self::Big => Self::Fullscreen,
      Self::Fullscreen => Self::Small,
    }
  }
  pub fn prev(self) -> Self {
    match self {
      Self::Small => Self::Fullscreen,
      Resolution::Medium => Resolution::Small,
      Self::Big => Resolution::Medium,
      Self::Fullscreen => Self::Big,
    }
  }
  pub fn size(self) -> (i32, i32) {
    match self {
      Resolution::Small => (640, 360),
      Resolution::Medium => (960, 540),
      Resolution::Big => (1280, 720),
      Resolution::Fullscreen => {
        let monitor = get_current_monitor();
        (get_monitor_width(monitor), get_monitor_height(monitor))
      }
    }
  }
}
