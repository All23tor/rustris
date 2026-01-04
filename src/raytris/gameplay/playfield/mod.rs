#[derive(Clone)]
pub struct Playfield;

impl Playfield {
  pub const WIDTH: usize = 10;
  pub const HEIGHT: usize = 40;
  pub const VISIBLE_HEIGHT: usize = 20;

  pub fn lost(&self) -> bool {
    false
  }
}
