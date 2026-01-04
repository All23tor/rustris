mod falling_piece;
mod line_clear_message;
mod next_queue;
mod tetromino;

use raylib::{
  RaylibHandle,
  color::Color,
  math::{Rectangle, Vector2},
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails,
    playfield::{
      falling_piece::FallingPiece, line_clear_message::LineClearMessage, next_queue::NextQueue,
      tetromino::Tetromino,
    },
  },
  settings::handling::HandlingSettings,
};

#[derive(Clone)]
pub struct Playfield {
  grid: [[Tetromino; Self::WIDTH as usize]; Self::HEIGHT as usize],
  next_queue: NextQueue,
  falling_piece: FallingPiece,
  holding_piece: Tetromino,
  can_swap: bool,
  has_lost: bool,
  last_move_rotation: bool,
  frames_since_drop: u32,
  lock_delay_frames: u32,
  lock_delay_resets: u32,
  frames_pressed: u32,
  combo: u32,
  score: u64,
  b2b: u32,
  message: LineClearMessage,
}

fn get_block(i: i32, j: i32, draw_d: &DrawingDetails) -> Rectangle {
  Rectangle {
    x: draw_d.position.x + i as f32 * draw_d.block_length,
    y: draw_d.position.y + (j - Playfield::VISIBLE_HEIGHT) as f32 * draw_d.block_length,
    width: draw_d.block_length,
    height: draw_d.block_length,
  }
}

fn draw_block_pretty(
  i: i32,
  j: i32,
  draw_d: &DrawingDetails,
  fill: Color,
  rld: &mut RaylibDrawHandle,
) {
  if fill.a == 0 {
    return;
  }

  let rec = get_block(i, j, draw_d);
  rld.draw_rectangle_rec(rec, fill);
  rld.draw_rectangle(
    (rec.x + draw_d.block_length / 3.0) as i32,
    (rec.y + draw_d.block_length / 3.0) as i32,
    (rec.width / 3.0) as i32,
    (rec.height / 3.0) as i32,
    DrawingDetails::DEFAULT_PRETTY_OUTLINE,
  );
  rld.draw_rectangle_lines_ex(
    rec,
    draw_d.block_length / 8.0,
    DrawingDetails::DEFAULT_PRETTY_OUTLINE,
  );
}

impl Playfield {
  pub const WIDTH: i32 = 10;
  pub const HEIGHT: i32 = 40;
  pub const VISIBLE_HEIGHT: i32 = 20;

  pub fn new() -> Self {
    Self {
      grid: [[Tetromino::Empty; _]; _],
      next_queue: NextQueue,
      falling_piece: FallingPiece,
      holding_piece: Tetromino::Empty,
      can_swap: true,
      has_lost: false,
      last_move_rotation: false,
      frames_since_drop: 0,
      lock_delay_frames: 0,
      lock_delay_resets: 0,
      frames_pressed: 0,
      combo: 0,
      score: 0,
      b2b: 0,
      message: LineClearMessage,
    }
  }

  pub fn restart(&mut self) {
    let last_score = self.score;
    *self = Playfield::new();
    self.score = last_score;
  }

  pub fn update(&mut self, _: &Controller, _: &HandlingSettings, _: &RaylibHandle) -> bool {
    false
  }

  pub fn draw(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    self.draw_tetrion(d, rld);
    // self.draw_tetrion_pieces(d, rld);
    // self.draw_hold_piece(d, rld);
    // self.draw_info(d, rld);
  }

  fn draw_tetrion(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let tetrion = Rectangle {
      x: d.position.x,
      y: d.position.y,
      width: d.block_length * Self::WIDTH as f32,
      height: d.block_length * Self::VISIBLE_HEIGHT as f32,
    };
    rld.draw_rectangle_rec(tetrion, DrawingDetails::TETRION_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      tetrion,
      d.block_length / 10.0,
      DrawingDetails::GRINDLINE_COLOR,
    );

    for mut rec in (1..Self::WIDTH).map(|i| get_block(i, Self::VISIBLE_HEIGHT, d)) {
      rec.x = rec.x.floor();
      rec.y = rec.y.floor();
      rld.draw_line_ex(
        Vector2 { x: rec.x, y: rec.y },
        Vector2 {
          x: rec.x,
          y: (rec.y + Self::VISIBLE_HEIGHT as f32 * d.block_length).floor(),
        },
        d.block_length / 10.0,
        DrawingDetails::GRINDLINE_COLOR,
      );
    }

    for mut rec in (1..Self::VISIBLE_HEIGHT).map(|j| get_block(0, j + Self::VISIBLE_HEIGHT, d)) {
      rec.x = rec.x.floor();
      rec.y = rec.y.floor();
      rld.draw_line_ex(
        Vector2 { x: rec.x, y: rec.y },
        Vector2 {
          x: (rec.x + d.block_length * Self::WIDTH as f32).floor(),
          y: rec.y,
        },
        d.block_length / 10.0,
        DrawingDetails::GRINDLINE_COLOR,
      );
    }

    for (i, j, mino) in self
      .grid
      .iter()
      .enumerate()
      .flat_map(|(j, row)| row.iter().enumerate().map(move |(i, mino)| (i, j, mino)))
    {
      draw_block_pretty(i as i32, j as i32, d, mino.color(), rld);
    }
  }

  pub fn lost(&self) -> bool {
    self.has_lost
  }
}
