use serde::{Deserialize, Serialize};
use std::{
  fs::{read_to_string, write},
  iter::zip,
  sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use raylib::{
  RaylibHandle,
  color::Color,
  ffi::KeyboardKey,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::settings::{handling::HandlingSettings, resolution::Resolution};

pub struct Menu {
  selected_option: Option,
}

#[derive(Clone, Default, Serialize, Deserialize)]
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

const OPTIONS: [Option; 3] = [
  Option::Resolution,
  Option::DelayedAutoShift,
  Option::SoftDropFrames,
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Option {
  Resolution,
  DelayedAutoShift,
  SoftDropFrames,
}

impl Option {
  fn next(self) -> Self {
    match self {
      Self::Resolution => Self::DelayedAutoShift,
      Self::DelayedAutoShift => Option::SoftDropFrames,
      Option::SoftDropFrames => Self::Resolution,
    }
  }
  fn prev(self) -> Self {
    match self {
      Self::Resolution => Option::SoftDropFrames,
      Self::DelayedAutoShift => Self::Resolution,
      Option::SoftDropFrames => Self::DelayedAutoShift,
    }
  }
}

impl Menu {
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
      -1
    } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
      1
    } else {
      return;
    };

    let mut config = config_mut();
    let hand_set = &mut config.handling_settings;
    match self.selected_option {
      Option::Resolution => {
        config.resolution = match change {
          1 => config.resolution.next(),
          -1 => config.resolution.prev(),
          _ => return,
        };

        let (width, height) = config.resolution.size();
        if rl.is_window_fullscreen() {
          rl.toggle_fullscreen();
        }
        rl.set_window_size(width, height);
        if config.resolution == Resolution::Fullscreen {
          rl.toggle_fullscreen();
        }
      }
      Option::DelayedAutoShift => {
        hand_set.das += hand_set.das.saturating_add_signed(change);
        hand_set.das = hand_set.das.min(20);
      }
      Option::SoftDropFrames => {
        hand_set.soft_drop += hand_set.das.saturating_add_signed(change);
        hand_set.soft_drop = hand_set.soft_drop.min(20);
      }
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    let (width, height) = config().resolution.size();
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
    let das = (
      "Delayed Auto Shift",
      format!("{}", config().handling_settings.das),
    );
    let soft_drop_frames = (
      "Soft Drop Frames",
      format!("{}", config().handling_settings.soft_drop),
    );

    let options = [resolution, das, soft_drop_frames];
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

impl Drop for Menu {
  fn drop(&mut self) {
    let Ok(serialized) = serde_json::to_string::<Config>(&config()) else {
      return;
    };

    write(SETTINGS_FILE_NAME, serialized).expect("Error writing");
  }
}
