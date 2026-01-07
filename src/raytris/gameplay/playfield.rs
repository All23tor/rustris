mod falling_piece;
mod line_clear_message;
mod next_queue;
mod tetromino;

use std::ops::Range;

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
      falling_piece::{FallingPiece, Shift},
      line_clear_message::{LineClearMessage, SpinType},
      next_queue::{NEXT_SIZE, NextQueue},
      tetromino::{Tetromino, TetrominoMap},
    },
  },
  settings::handling::HandlingSettings,
};

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 40;
pub const VISIBLE_HEIGHT: i32 = 20;
const INITIAL_X_POSITION: i8 = (WIDTH as i8 - 1) / 2;
const INITIAL_Y_POSITION: i8 = VISIBLE_HEIGHT as i8 - 1;

type Grid = [[Tetromino; WIDTH as usize]; HEIGHT as usize];

#[derive(Clone)]
pub struct Playfield {
  grid: Grid,
  next_queue: NextQueue,
  falling_piece: FallingPiece,
  holding_piece: Tetromino,
  can_swap: bool,
  has_lost: bool,
  last_move_rotation: bool,
  frames_since_drop: u32,
  lock_delay_frames: u32,
  lock_delay_resets: u32,
  frames_pressed: i32,
  combo: u32,
  score: u64,
  b2b: u32,
  message: Option<LineClearMessage>,
}

impl Playfield {
  pub fn new() -> Self {
    let mut next_queue = NextQueue::new();
    let falling_piece = spawn_tetromino(next_queue.next_tetromino());
    Self {
      grid: [[Tetromino::Empty; _]; _],
      next_queue,
      falling_piece,
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
      message: None,
    }
  }

  pub fn restart(&mut self) {
    let last_score = self.score;
    *self = Playfield::new();
    self.score = last_score;
  }

  pub fn update(&mut self, c: &Controller, h: &HandlingSettings, rl: &RaylibHandle) -> bool {
    if self.has_lost {
      return false;
    }

    self.handle_swap(c, rl);

    self.frames_since_drop += 1;
    self.lock_delay_frames += 1;
    if let Some(message) = &mut self.message {
      message.timer -= 1;
      if message.timer == 0 {
        self.message = None;
      }
    }

    self.handle_shifts(c, h, rl);
    // self.handle_rotations(c, rl);
    // self.handle_drops(c, h, rl)
    false
  }

  fn handle_swap(&mut self, c: &Controller, rl: &RaylibHandle) {
    if !(c.swap)(rl) || !self.can_swap {
      return;
    }

    let current_tetromino = self.falling_piece.tetromino;
    self.falling_piece = if self.holding_piece != Tetromino::Empty {
      spawn_tetromino(self.holding_piece)
    } else {
      spawn_tetromino(self.next_queue.next_tetromino())
    };
    self.holding_piece = current_tetromino;
    self.can_swap = false;
    self.frames_since_drop = 0;
    self.lock_delay_frames = 0;
    self.lock_delay_resets = 0;
    self.last_move_rotation = false;
  }

  fn handle_shifts(&mut self, c: &Controller, h: &HandlingSettings, rl: &RaylibHandle) {
    let mut try_shifting = |shift| {
      let mut shifted_piece = self.falling_piece.clone();
      shifted_piece.shift(shift);
      if valid_position(&self.grid, &shifted_piece) {
        self.falling_piece = shifted_piece;
        self.lock_delay_frames = 0;
        self.lock_delay_resets += 1;
        self.last_move_rotation = false;
      }
    };
    if (c.left)(rl) {
      try_shifting(Shift::Left);
    } else if (c.right)(rl) {
      try_shifting(Shift::Right);
    }

    let mut try_das = |shift| {
      let mut shifted_piece = self.falling_piece.clone();
      shifted_piece.shift(shift);
      while valid_position(&self.grid, &shifted_piece) {
        self.falling_piece = shifted_piece.clone();
        self.lock_delay_frames = 0;
        self.lock_delay_resets += 1;
        shifted_piece.shift(shift);
      }
    };

    if (c.left_das)(rl) {
      self.frames_pressed = self.frames_pressed.max(0) + 1;
      if self.frames_pressed > h.das {
        try_das(Shift::Left);
      }
    } else if (c.right_das)(rl) {
      self.frames_pressed = self.frames_pressed.min(0) - 1;
      if self.frames_pressed < -h.das {
        try_das(Shift::Right);
      }
    } else {
      self.frames_pressed = 0;
    }
  }

  pub fn draw(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    self.draw_tetrion(d, rld);
    self.draw_tetrion_pieces(d, rld);
    self.draw_next_queue(d, rld);
    self.draw_hold_piece(d, rld);
    self.draw_info(d, rld);
  }

  fn draw_tetrion(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let tetrion = Rectangle {
      x: d.position.x,
      y: d.position.y,
      width: d.block_length * WIDTH as f32,
      height: d.block_length * VISIBLE_HEIGHT as f32,
    };
    rld.draw_rectangle_rec(tetrion, DrawingDetails::TETRION_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      tetrion,
      d.block_length / 10.0,
      DrawingDetails::GRINDLINE_COLOR,
    );

    for mut rec in (1..WIDTH).map(|i| get_block(i, VISIBLE_HEIGHT, d)) {
      rec.x = rec.x.floor();
      rec.y = rec.y.floor();
      rld.draw_line_ex(
        Vector2 { x: rec.x, y: rec.y },
        Vector2 {
          x: rec.x,
          y: (rec.y + VISIBLE_HEIGHT as f32 * d.block_length).floor(),
        },
        d.block_length / 10.0,
        DrawingDetails::GRINDLINE_COLOR,
      );
    }

    for mut rec in (1..VISIBLE_HEIGHT).map(|j| get_block(0, j + VISIBLE_HEIGHT, d)) {
      rec.x = rec.x.floor();
      rec.y = rec.y.floor();
      rld.draw_line_ex(
        Vector2 { x: rec.x, y: rec.y },
        Vector2 {
          x: (rec.x + d.block_length * WIDTH as f32).floor(),
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

  fn draw_tetrion_pieces(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let mut ghost_piece = self.falling_piece.clone();
    ghost_piece.fall();
    while valid_position(&self.grid, &ghost_piece) {
      ghost_piece.fall();
    }
    ghost_piece.unfall();

    draw_piece(
      &ghost_piece.map,
      Color::GRAY,
      ghost_piece.x as i32,
      ghost_piece.y as i32,
      d,
      rld,
    );

    draw_piece(
      &self.falling_piece.map,
      self.falling_piece.tetromino.color(),
      self.falling_piece.x as i32,
      self.falling_piece.y as i32,
      d,
      rld,
    );

    const X_DANGER_RANGE: Range<usize> = WIDTH as usize / 2 - 2..WIDTH as usize / 2 + 2;
    const Y_DANGER_RANGE: Range<usize> =
      INITIAL_Y_POSITION as usize..INITIAL_Y_POSITION as usize + 5;

    let is_in_danger = X_DANGER_RANGE.clone().all(|x| {
      Y_DANGER_RANGE
        .clone()
        .all(|y| self.grid[y][x] != Tetromino::Empty)
    });
    if is_in_danger {
      draw_piece_danger(self.next_queue[0], d, rld);
    }
  }

  fn draw_next_queue(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let mut background = get_block(WIDTH + 1, VISIBLE_HEIGHT + 2, d);
    background.width = d.block_length * 6.0;
    background.height = d.block_length * (3 * NEXT_SIZE + 1) as f32;
    rld.draw_rectangle_rec(background, DrawingDetails::PIECES_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      background,
      d.block_length / 4.0,
      DrawingDetails::PIECE_BOX_COLOR,
    );

    let text_rect = get_block(WIDTH + 1, VISIBLE_HEIGHT, d);
    rld.draw_text(
      "NEXT",
      text_rect.x as i32,
      text_rect.y as i32,
      d.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );

    for id in 0..NEXT_SIZE {
      draw_piece(
        &self.next_queue[id].initial_map(),
        self.next_queue[id].color(),
        WIDTH + 3,
        3 * (id as i32 + 1) + VISIBLE_HEIGHT + 1,
        d,
        rld,
      );
    }
  }

  fn draw_hold_piece(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let text_rect = get_block(-7, VISIBLE_HEIGHT, d);
    rld.draw_text(
      "HOLD",
      text_rect.x as i32,
      text_rect.y as i32,
      d.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );
    let mut background = get_block(-7, VISIBLE_HEIGHT + 2, d);
    background.width = d.block_length * 6.0;
    background.height = d.block_length * 4.0;
    rld.draw_rectangle_rec(background, DrawingDetails::PIECES_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      background,
      d.block_length / 4.0,
      DrawingDetails::PIECE_BOX_COLOR,
    );

    let color = if self.can_swap {
      self.holding_piece.color()
    } else {
      DrawingDetails::UNAVAILABLE_HOLD_PIECE_COLOR
    };

    draw_piece(
      &self.holding_piece.initial_map(),
      color,
      -5,
      4 + VISIBLE_HEIGHT,
      d,
      rld,
    );
  }

  fn draw_info(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    if let Some(message) = &self.message {
      let text_rect = get_block(DrawingDetails::LEFT_BORDER, HEIGHT - 4, d);
      let (msg, mut color) = message.message.info();
      let alpha = (255.0 * message.timer as f32) / LineClearMessage::DURATION as f32;
      color.a = alpha as u8;
      rld.draw_text(
        msg,
        text_rect.x as i32,
        text_rect.y as i32,
        d.font_size,
        color,
      );

      if let Some(spin_type) = message.spin_type {
        let mut spin_color = Tetromino::T.color();
        spin_color.a = alpha as u8;
        let spin_rect = get_block(DrawingDetails::LEFT_BORDER, HEIGHT - 6, d);
        rld.draw_text(
          "TSPIN",
          spin_rect.x as i32,
          spin_rect.y as i32,
          d.font_size,
          spin_color,
        );
        if spin_type == SpinType::Mini {
          let mini_rect = get_block(DrawingDetails::LEFT_BORDER, HEIGHT - 7, d);
          rld.draw_text(
            "MINI",
            mini_rect.x as i32,
            mini_rect.y as i32,
            d.font_size_small,
            spin_color,
          );
        }
      }
    }

    if self.combo >= 2 {
      let combo_rect = get_block(DrawingDetails::LEFT_BORDER, HEIGHT - 10, d);
      rld.draw_text(
        "COMBO ",
        combo_rect.x as i32,
        combo_rect.y as i32,
        d.font_size,
        Color::BLUE,
      );
      rld.draw_text(
        &format!("{}", self.combo),
        combo_rect.x as i32 + rld.measure_text("COMBO ", d.font_size),
        combo_rect.y as i32,
        d.font_size,
        Color::BLUE,
      );
    }

    if self.b2b >= 2 {
      let b2b_rect = get_block(DrawingDetails::LEFT_BORDER, HEIGHT - 12, d);
      rld.draw_text(
        "B2B ",
        b2b_rect.x as i32,
        b2b_rect.y as i32,
        d.font_size,
        Color::BLUE,
      );
      rld.draw_text(
        &format!("{}", self.b2b - 1),
        b2b_rect.x as i32 + rld.measure_text("B2B ", d.font_size),
        b2b_rect.y as i32,
        d.font_size,
        Color::BLUE,
      );
    }

    let score_rect = get_block(WIDTH + 1, HEIGHT - 2, d);
    rld.draw_text(
      &format!("{:09}", self.score),
      score_rect.x as i32,
      (score_rect.y + d.block_length * 0.5) as i32,
      d.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );
  }

  pub fn lost(&self) -> bool {
    self.has_lost
  }
}

fn spawn_tetromino(tetromino: Tetromino) -> FallingPiece {
  FallingPiece::new(tetromino, INITIAL_X_POSITION, INITIAL_Y_POSITION)
}

fn valid_position(grid: &Grid, piece: &FallingPiece) -> bool {
  piece.map.0.iter().all(|(cx, cy)| {
    let x = (cx + piece.x) as i32;
    let y = (cy + piece.y) as i32;
    valid_mino(x, y) && grid[y as usize][x as usize] == Tetromino::Empty
  })
}

fn valid_mino(x: i32, y: i32) -> bool {
  (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y)
}

fn get_block(i: i32, j: i32, d: &DrawingDetails) -> Rectangle {
  Rectangle {
    x: d.position.x + i as f32 * d.block_length,
    y: d.position.y + (j - VISIBLE_HEIGHT) as f32 * d.block_length,
    width: d.block_length,
    height: d.block_length,
  }
}

fn draw_block_pretty(i: i32, j: i32, d: &DrawingDetails, fill: Color, rld: &mut RaylibDrawHandle) {
  if fill.a == 0 {
    return;
  }

  let rec = get_block(i, j, d);
  rld.draw_rectangle_rec(rec, fill);
  rld.draw_rectangle(
    (rec.x + d.block_length / 3.0) as i32,
    (rec.y + d.block_length / 3.0) as i32,
    (rec.width / 3.0) as i32,
    (rec.height / 3.0) as i32,
    DrawingDetails::DEFAULT_PRETTY_OUTLINE,
  );
  rld.draw_rectangle_lines_ex(
    rec,
    d.block_length / 8.0,
    DrawingDetails::DEFAULT_PRETTY_OUTLINE,
  );
}

fn draw_piece(
  map: &TetrominoMap,
  color: Color,
  x: i32,
  y: i32,
  d: &DrawingDetails,
  rld: &mut RaylibDrawHandle,
) {
  for (cx, cy) in map.0 {
    draw_block_pretty(cx as i32 + x, cy as i32 + y, d, color, rld);
  }
}

fn draw_piece_danger(tetromino: Tetromino, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
  for (cx, cy) in tetromino.initial_map().0 {
    let x = cx as i32 + INITIAL_X_POSITION as i32;
    let y = cy as i32 + INITIAL_Y_POSITION as i32;
    draw_block_danger(x, y, d, rld);
  }
}

fn draw_block_danger(i: i32, j: i32, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
  let rec = get_block(i, j, d);
  let &Rectangle {
    x,
    y,
    width,
    height,
  } = &rec;

  const SOFT_RED: Color = Color::new(255, 0, 0, 150);
  rld.draw_rectangle_lines_ex(rec, d.block_length / 8.0, SOFT_RED);
  rld.draw_line_ex(
    Vector2::new(x + width * 0.25, y + height * 0.25),
    Vector2::new(x + width * 0.75, y + height * 0.75),
    d.block_length * 0.1,
    Color::RED,
  );
  rld.draw_line_ex(
    Vector2::new(x + width * 0.75, y + height * 0.25),
    Vector2::new(x + width * 0.25, y + height * 0.75),
    d.block_length * 0.1,
    SOFT_RED,
  );
}
