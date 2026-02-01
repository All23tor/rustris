mod falling_piece;
mod next_queue;
pub mod tetromino;

use std::{iter::zip, ops::Range, time::Duration};

use raylib::{
  RaylibHandle,
  color::Color,
  math::{Rectangle, Vector2},
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{
    Controller, DrawingDetails,
    line_clear_message::SpinType,
    playfield::{
      falling_piece::{FallingPiece, Orientation, RotationType, Shift},
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
const INITIAL_Y_POSITION: i8 = VISIBLE_HEIGHT as i8;

type Grid = [[Option<Tetromino>; WIDTH as usize]; HEIGHT as usize];

pub struct UpdateInfo {
  pub cleared_lines: u32,
  pub spin: Option<(Tetromino, SpinType)>,
  pub is_all_clear: bool,
}

#[derive(Clone)]
pub struct Playfield {
  grid: Grid,
  next_queue: NextQueue,
  falling_piece: FallingPiece,
  holding_piece: Option<Tetromino>,
  can_swap: bool,
  last_move_rotation: bool,
  last_drop: Duration,
  lock_delay: Duration,
  lock_delay_resets: u32,
  das_press: Option<(Shift, Duration)>,
  has_lost: bool,
}

impl Playfield {
  pub fn new() -> Self {
    let mut next_queue = NextQueue::new();
    let falling_piece = spawn_tetromino(next_queue.next_tetromino());
    Self {
      grid: [[None; _]; _],
      next_queue,
      falling_piece,
      holding_piece: None,
      can_swap: true,
      last_move_rotation: false,
      last_drop: Duration::ZERO,
      lock_delay: Duration::ZERO,
      lock_delay_resets: 0,
      das_press: None,
      has_lost: false,
    }
  }

  pub fn has_lost(&self) -> bool {
    self.has_lost
  }

  pub fn update(
    &mut self,
    c: &Controller,
    h: &HandlingSettings,
    dt: Duration,
    rl: &RaylibHandle,
  ) -> Option<UpdateInfo> {
    if self.has_lost {
      return None;
    }

    self.last_drop += dt;
    self.lock_delay += dt;

    self.handle_swap(c, rl);
    self.handle_shifts(c, h, dt, rl);
    self.handle_rotations(c, rl);
    self.handle_drops(c, h, rl)
  }

  fn handle_swap(&mut self, c: &Controller, rl: &RaylibHandle) {
    if !(c.swap)(rl) || !self.can_swap {
      return;
    }

    let current_tetromino = self.falling_piece.tetromino;
    self.falling_piece = spawn_tetromino(
      self
        .holding_piece
        .unwrap_or_else(|| self.next_queue.next_tetromino()),
    );
    self.holding_piece = Some(current_tetromino);
    self.can_swap = false;
    self.last_drop = Duration::ZERO;
    self.lock_delay = Duration::ZERO;
    self.lock_delay_resets = 0;
    self.last_move_rotation = false;
  }

  fn handle_shifts(
    &mut self,
    c: &Controller,
    h: &HandlingSettings,
    dt: Duration,
    rl: &RaylibHandle,
  ) {
    let mut try_shifting = |shift| {
      let mut shifted_piece = self.falling_piece.clone();
      shifted_piece.shift(shift);
      if valid_position(&self.grid, &shifted_piece) {
        self.falling_piece = shifted_piece;
        self.lock_delay = Duration::ZERO;
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
      let duration = self
        .das_press
        .filter(|&(s_shift, _)| s_shift == shift)
        .map(|(_, duration)| duration)
        .unwrap_or_default()
        + dt;

      self.das_press = Some((shift, duration));

      if duration < h.das {
        return;
      }

      let mut shifted_piece = self.falling_piece.clone();
      shifted_piece.shift(shift);
      while valid_position(&self.grid, &shifted_piece) {
        self.falling_piece = shifted_piece.clone();
        self.lock_delay = Duration::ZERO;
        self.lock_delay_resets += 1;
        shifted_piece.shift(shift);
      }
    };

    if (c.left_das)(rl) {
      try_das(Shift::Left);
    } else if (c.right_das)(rl) {
      try_das(Shift::Right);
    } else {
      self.das_press = None;
    }
  }

  fn handle_rotations(&mut self, c: &Controller, rl: &RaylibHandle) {
    let rotation_type = if (c.clockwise)(rl) {
      RotationType::Clockwise
    } else if (c.counter_clockwise)(rl) {
      RotationType::CounterClockwise
    } else if (c.one_eighty)(rl) {
      RotationType::OneEighty
    } else {
      return;
    };

    let mut rotated_piece = self.falling_piece.clone();
    rotated_piece.rotate(rotation_type);

    let Some(offset) = zip(self.falling_piece.offsets(), rotated_piece.offsets())
      .map(|(p1, p2)| (p1.0 - p2.0, p1.1 - p2.1))
      .find(|&offset| {
        let mut translated_piece = rotated_piece.clone();
        translated_piece.translate(offset);
        valid_position(&self.grid, &translated_piece)
      })
    else {
      return;
    };

    rotated_piece.translate(offset);
    self.falling_piece = rotated_piece;
    self.lock_delay = Duration::ZERO;
    self.lock_delay_resets += 1;
    self.last_move_rotation = true;
  }

  fn handle_drops(
    &mut self,
    c: &Controller,
    h: &HandlingSettings,
    rl: &RaylibHandle,
  ) -> Option<UpdateInfo> {
    if (c.hard_drop)(rl) {
      let mut fallen = self.falling_piece.clone();
      fallen.fall();
      while valid_position(&self.grid, &fallen) {
        self.falling_piece = fallen.clone();
        fallen.fall();
        self.last_move_rotation = false;
      }
      return Some(self.solidify_piece());
    }

    let soft_fall = (c.soft_drop)(rl) && self.last_drop >= h.soft_drop;
    let gravity_fall = self.last_drop >= h.gravity;
    let is_fall_step = soft_fall || gravity_fall;
    if is_fall_step {
      self.last_drop = Duration::ZERO
    };

    let mut fallen_piece = self.falling_piece.clone();
    fallen_piece.fall();
    let can_fall = valid_position(&self.grid, &fallen_piece);
    let can_wait = self.lock_delay < h.lock_delay;
    let can_reset = self.lock_delay_resets < h.lock_delay_resets;
    if !can_fall && (!can_wait || !can_reset) {
      return Some(self.solidify_piece());
    }

    if can_fall && is_fall_step {
      self.last_move_rotation = false;
      self.falling_piece.fall();
      self.lock_delay = Duration::ZERO;
      self.lock_delay_resets = 0;
    }

    None
  }

  fn is_spin(piece: &FallingPiece, grid: &Grid) -> Option<SpinType> {
    // Disable this check to allow all spins
    if piece.tetromino != Tetromino::T {
      return None;
    }

    let mut corners = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    corners.rotate_right(match piece.orientation {
      Orientation::Up => 0,
      Orientation::Right => 1,
      Orientation::Down => 2,
      Orientation::Left => 3,
    });

    let front_count = corners[0..2]
      .iter()
      .filter(|&&(cx, cy)| {
        let x = piece.x as i32 + cx;
        let y = piece.y as i32 + cy;
        !valid_mino(x, y) || grid[y as usize][x as usize].is_some()
      })
      .count();
    let back_count = corners[2..4]
      .iter()
      .filter(|&&(cx, cy)| {
        let x = piece.x as i32 + cx;
        let y = piece.y as i32 + cy;
        !valid_mino(x, y) || grid[y as usize][x as usize].is_some()
      })
      .count();

    if front_count + back_count < 3 {
      None
    } else if front_count == 2 {
      Some(SpinType::Proper)
    } else {
      Some(SpinType::Mini)
    }
  }

  fn solidify_piece(&mut self) -> UpdateInfo {
    let mut topped_out = true;

    for (cx, cy) in self.falling_piece.map {
      let x = cx as i32 + self.falling_piece.x as i32;
      let y = cy as i32 + self.falling_piece.y as i32;
      self.grid[y as usize][x as usize] = Some(self.falling_piece.tetromino);

      if y < VISIBLE_HEIGHT {
        topped_out = false;
      }
    }

    let tetromino = self.falling_piece.tetromino;
    let spin_type = if self.last_move_rotation {
      Self::is_spin(&self.falling_piece, &self.grid)
    } else {
      None
    };

    let mut cleared_lines = 0;
    for row_idx in (0..HEIGHT as usize).rev() {
      if self.grid[row_idx].iter().all(|&m| m.is_some()) {
        self.grid.copy_within(row_idx + 1..HEIGHT as usize, row_idx);
        self.grid[HEIGHT as usize - 1].fill(None);
        cleared_lines += 1;
      }
    }

    let is_all_clear = self.grid.as_flattened().iter().all(|mino| mino.is_none());

    let next_tetromino = self.next_queue.next_tetromino();
    self.falling_piece = spawn_tetromino(next_tetromino);
    self.last_drop = Duration::ZERO;
    self.lock_delay = Duration::ZERO;
    self.lock_delay_resets = 0;
    self.can_swap = true;

    let can_spawn_piece = self.falling_piece.map.iter().all(|&(cx, cy)| {
      let x = cx as i32 + self.falling_piece.x as i32;
      let y = cy as i32 + self.falling_piece.y as i32;
      self.grid[y as usize][x as usize].is_none()
    });

    self.has_lost = topped_out || !can_spawn_piece;
    let spin = spin_type.map(|spin_type| (tetromino, spin_type));

    UpdateInfo {
      cleared_lines,
      spin,
      is_all_clear,
    }
  }

  pub fn draw(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    self.draw_grid(d, rld);
    self.draw_main_pieces(d, rld);
    self.draw_next_queue(d, rld);
    self.draw_hold_piece(d, rld);
  }

  fn draw_grid(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let grid_rec = Rectangle {
      x: d.position.x,
      y: d.position.y,
      width: d.block_length * WIDTH as f32,
      height: d.block_length * VISIBLE_HEIGHT as f32,
    };
    rld.draw_rectangle_rec(grid_rec, DrawingDetails::GRID_BACKGROUND_COLOR);

    let line_width = d.block_length / 10.0;
    rld.draw_rectangle_lines_ex(grid_rec, line_width, DrawingDetails::GRIDLINE_COLOR);

    for Vector2 { x, y } in (0..WIDTH).map(|i| get_block(i, VISIBLE_HEIGHT - 1, d)) {
      let p0 = Vector2 {
        x: x.floor(),
        y: y.floor(),
      };
      let p1 = Vector2 {
        x: x.floor(),
        y: (y + VISIBLE_HEIGHT as f32 * d.block_length).floor(),
      };
      rld.draw_line_ex(p0, p1, line_width, DrawingDetails::GRIDLINE_COLOR);
    }
    for Vector2 { x, y } in (0..VISIBLE_HEIGHT).map(|j| get_block(0, j, d)) {
      let p0 = Vector2 {
        x: x.floor(),
        y: y.floor(),
      };
      let p1 = Vector2 {
        x: (x + d.block_length * WIDTH as f32).floor(),
        y: y.floor(),
      };
      rld.draw_line_ex(p0, p1, line_width, DrawingDetails::GRIDLINE_COLOR);
    }

    for (j, row) in self.grid.iter().enumerate() {
      for (i, mino) in row.iter().enumerate() {
        let color = mino.map_or(Color::BLANK, |t| t.color());
        draw_block_pretty(i as i32, j as i32, d, color, rld);
      }
    }
  }

  fn draw_main_pieces(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
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

    const X_DANGER: Range<usize> = INITIAL_X_POSITION as usize - 2..INITIAL_X_POSITION as usize + 2;
    const Y_DANGER: Range<usize> = INITIAL_Y_POSITION as usize - 5..INITIAL_Y_POSITION as usize;

    let mut danger_zone = X_DANGER.flat_map(|x| Y_DANGER.map(move |y| (x, y)));
    if danger_zone.any(|(x, y)| self.grid[y][x].is_some()) {
      draw_piece_danger(self.next_queue.peek(), d, rld);
    }
  }

  fn draw_next_queue(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let Vector2 { x: bg_x, y: bg_y } = get_block(WIDTH + 1, VISIBLE_HEIGHT - 3, d);
    let background = Rectangle {
      x: bg_x,
      y: bg_y,
      width: d.block_length * 6.0,
      height: d.block_length * (3 * NEXT_SIZE + 1) as f32,
    };
    rld.draw_rectangle_rec(background, DrawingDetails::PIECES_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      background,
      d.block_length / 4.0,
      DrawingDetails::PIECE_BOX_COLOR,
    );

    let text = get_block(WIDTH + 1, VISIBLE_HEIGHT - 1, d);
    rld.draw_text(
      "NEXT",
      text.x as i32,
      text.y as i32,
      d.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );

    for (id, tetromino) in self.next_queue.queue().enumerate() {
      draw_piece(
        &tetromino.initial_map(),
        tetromino.color(),
        WIDTH + 3,
        -3 * id as i32 + VISIBLE_HEIGHT - 5,
        d,
        rld,
      );
    }
  }

  fn draw_hold_piece(&self, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
    let text = get_block(-7, VISIBLE_HEIGHT - 1, d);
    rld.draw_text(
      "HOLD",
      text.x as i32,
      text.y as i32,
      d.font_size,
      DrawingDetails::INFO_TEXT_COLOR,
    );
    let Vector2 { x: bg_x, y: bg_y } = get_block(-7, VISIBLE_HEIGHT - 3, d);
    let background = Rectangle {
      x: bg_x,
      y: bg_y,
      width: d.block_length * 6.0,
      height: d.block_length * 4.0,
    };
    rld.draw_rectangle_rec(background, DrawingDetails::PIECES_BACKGROUND_COLOR);
    rld.draw_rectangle_lines_ex(
      background,
      d.block_length / 4.0,
      DrawingDetails::PIECE_BOX_COLOR,
    );

    let Some(holding_piece) = self.holding_piece else {
      return;
    };

    let color = if self.can_swap {
      holding_piece.color()
    } else {
      DrawingDetails::UNAVAILABLE_HOLD_PIECE_COLOR
    };

    draw_piece(
      &holding_piece.initial_map(),
      color,
      -5,
      -5 + VISIBLE_HEIGHT,
      d,
      rld,
    );
  }
}

fn spawn_tetromino(tetromino: Tetromino) -> FallingPiece {
  FallingPiece::new(tetromino, INITIAL_X_POSITION, INITIAL_Y_POSITION)
}

fn valid_position(grid: &Grid, piece: &FallingPiece) -> bool {
  piece.map.iter().all(|(cx, cy)| {
    let x = (cx + piece.x) as i32;
    let y = (cy + piece.y) as i32;
    valid_mino(x, y) && grid[y as usize][x as usize].is_none()
  })
}

fn valid_mino(x: i32, y: i32) -> bool {
  (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y)
}

fn get_block(i: i32, j: i32, d: &DrawingDetails) -> Vector2 {
  let y_offset = (VISIBLE_HEIGHT as f32 - 1.0) * d.block_length;
  Vector2 {
    x: d.position.x + i as f32 * d.block_length,
    y: d.position.y + y_offset - j as f32 * d.block_length,
  }
}

fn draw_block_pretty(i: i32, j: i32, d: &DrawingDetails, fill: Color, rld: &mut RaylibDrawHandle) {
  if fill.a == 0 {
    return;
  }

  let Vector2 { x, y } = get_block(i, j, d);
  let (width, height) = (d.block_length, d.block_length);
  let rec = Rectangle {
    x,
    y,
    width,
    height,
  };

  rld.draw_rectangle_rec(rec, fill);
  rld.draw_rectangle(
    (x + d.block_length / 3.0) as i32,
    (y + d.block_length / 3.0) as i32,
    (width / 3.0) as i32,
    (height / 3.0) as i32,
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
  for &(cx, cy) in map {
    draw_block_pretty(cx as i32 + x, cy as i32 + y, d, color, rld);
  }
}

fn draw_piece_danger(tetromino: Tetromino, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
  for (cx, cy) in tetromino.initial_map() {
    let x = cx as i32 + INITIAL_X_POSITION as i32;
    let y = cy as i32 + INITIAL_Y_POSITION as i32;
    draw_block_danger(x, y, d, rld);
  }
}

fn draw_block_danger(i: i32, j: i32, d: &DrawingDetails, rld: &mut RaylibDrawHandle) {
  let Vector2 { x, y } = get_block(i, j, d);
  let (width, height) = (d.block_length, d.block_length);
  let rec = Rectangle {
    x,
    y,
    width,
    height,
  };

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
