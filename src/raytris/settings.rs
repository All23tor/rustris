use raylib::{
  RaylibHandle,
  color::Color,
  consts::KeyboardKey,
  prelude::{RaylibDraw, RaylibDrawHandle},
  window::{get_current_monitor, get_monitor_height, get_monitor_width},
};
use serde::{Deserialize, Serialize};
use std::{
  fs::{read_to_string, write},
  iter::zip,
  sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
  time::Duration,
};

use super::gameplay::HandlingSettings;

pub struct SettingsMenu {
  selected_option: Option,
}

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

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
  pub resolution: Resolution,
  pub handling_settings: HandlingSettings,
}

const SETTINGS_FILE_NAME: &str = "settings.raytris";

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
  let config_text = read_to_string(SETTINGS_FILE_NAME).ok();
  RwLock::new(
    config_text
      .and_then(|text| serde_json::from_str(&text).ok())
      .unwrap_or_default(),
  )
});

pub fn config() -> RwLockReadGuard<'static, Config> {
  CONFIG.read().expect("Lock poisoned")
}

fn config_mut() -> RwLockWriteGuard<'static, Config> {
  CONFIG.write().expect("Lock poisoned")
}

const OPTIONS: [Option; 3] = [Option::Resolution, Option::Das, Option::SoftDrop];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Option {
  Resolution,
  Das,
  SoftDrop,
}

impl Option {
  fn next(self) -> Self {
    match self {
      Self::Resolution => Self::Das,
      Self::Das => Option::SoftDrop,
      Option::SoftDrop => Self::Resolution,
    }
  }
  fn prev(self) -> Self {
    match self {
      Self::Resolution => Option::SoftDrop,
      Self::Das => Self::Resolution,
      Option::SoftDrop => Self::Das,
    }
  }
}

enum Direction {
  Left,
  Right,
}

impl SettingsMenu {
  pub fn new() -> Self {
    Self {
      selected_option: Option::Resolution,
    }
  }

  pub fn update(&mut self, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
      self.selected_option = self.selected_option.next();
    } else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
      self.selected_option = self.selected_option.prev();
    }

    let change = if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
      Direction::Left
    } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
      Direction::Right
    } else {
      return;
    };

    let Config {
      resolution,
      handling_settings: hs,
    } = &mut *config_mut();
    match self.selected_option {
      Option::Resolution => {
        *resolution = match change {
          Direction::Left => resolution.prev(),
          Direction::Right => resolution.next(),
        };

        let (width, height) = resolution.size();
        if rl.is_window_fullscreen() {
          rl.toggle_fullscreen();
        }
        rl.set_window_size(width, height);
        if *resolution == Resolution::Fullscreen {
          rl.toggle_fullscreen();
        }
      }

      Option::Das => {
        hs.das = match change {
          Direction::Left => hs.das.saturating_sub(Duration::from_millis(10)),
          Direction::Right => hs.das.saturating_add(Duration::from_millis(10)),
        };
        hs.das = hs.das.min(Duration::from_millis(330));
      }
      Option::SoftDrop => {
        hs.soft_drop = match change {
          Direction::Left => hs.soft_drop.saturating_sub(Duration::from_millis(10)),
          Direction::Right => hs.soft_drop.saturating_add(Duration::from_millis(10)),
        };
        hs.soft_drop = hs.soft_drop.min(Duration::from_millis(330));
      }
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    let Config {
      resolution,
      handling_settings: hs,
    } = &mut *config_mut();
    let (width, height) = resolution.size();
    let font_size = height as f32 / 12.0;
    let font_size_big = height as f32 / 4.0;

    rld.clear_background(Color::LIGHTGRAY);
    rld.draw_text(
      "SETTINGS",
      (width - rld.measure_text("SETTINGS", font_size_big as i32)) / 2,
      height / 2 - font_size_big as i32 - font_size as i32,
      font_size_big as i32,
      Color::RED,
    );

    let resolution = ("Resolution", format!("{} x {}", width, height));
    let das = ("DAS", format!("{:0.2}", hs.das.as_secs_f32()));
    let soft_drop = ("Soft Drop", format!("{:0.2}", hs.soft_drop.as_secs_f32()));

    let options = [resolution, das, soft_drop];
    for (i, (option, (name, value))) in zip(OPTIONS, options).enumerate() {
      let color = if self.selected_option == option {
        Color::BLUE
      } else {
        Color::BLACK
      };

      rld.draw_text(
        name,
        (width as f32 / 8.0) as i32,
        (height as f32 / 2.0) as i32 + i as i32 * font_size as i32,
        font_size as i32,
        color,
      );
      rld.draw_text(
        &value,
        (width as f32 / 1.5) as i32,
        (height as f32 / 2.0) as i32 + i as i32 * font_size as i32,
        font_size as i32,
        color,
      );
    }
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
  }
}

impl Drop for SettingsMenu {
  fn drop(&mut self) {
    let Ok(serialized) = serde_json::to_string::<Config>(&config()) else {
      return;
    };

    write(SETTINGS_FILE_NAME, serialized).expect("Error writing");
  }
}
