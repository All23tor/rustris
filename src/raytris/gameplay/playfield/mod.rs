use raylib::{RaylibHandle, prelude::RaylibDrawHandle};

use crate::raytris::{
  gameplay::{Controller, DrawingDetails},
  settings::handling::HandlingSettings,
};

#[derive(Clone)]
pub struct Playfield;

impl Playfield {
  pub const WIDTH: usize = 10;
  pub const HEIGHT: usize = 40;
  pub const VISIBLE_HEIGHT: usize = 20;

  pub fn restart(&mut self) {}

  pub fn update(&mut self, _: &Controller, _: &HandlingSettings, _: &RaylibHandle) -> bool {
    false
  }

  pub fn draw(&self, _: &DrawingDetails, _: &RaylibDrawHandle) {}

  pub fn lost(&self) -> bool {
    false
  }
}
