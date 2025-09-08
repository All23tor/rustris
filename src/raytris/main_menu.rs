use raylib::{
  RaylibHandle,
  color::Color,
  ffi::{KeyboardKey, Rectangle},
  prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Option {
  SinglePlayer,
  TwoPlayer,
  Settings,
  Exit,
}

impl Option {
  fn into_str(self) -> &'static str {
    match self {
      Self::SinglePlayer => "Single Player",
      Self::TwoPlayer => "Two Player",
      Self::Settings => "Settings",
      Self::Exit => "Exit",
    }
  }

  fn next(self) -> Self {
    match self {
      Self::SinglePlayer => Self::TwoPlayer,
      Self::TwoPlayer => Self::Settings,
      Self::Settings => Self::SinglePlayer,
      Self::Exit => Self::Exit,
    }
  }

  fn prev(self) -> Self {
    match self {
      Self::SinglePlayer => Self::Settings,
      Self::TwoPlayer => Self::SinglePlayer,
      Self::Settings => Self::TwoPlayer,
      Self::Exit => Self::Exit,
    }
  }

  const OPTIONS: [Option; 3] = [Option::SinglePlayer, Option::TwoPlayer, Option::Settings];
}

pub struct MainMenu {
  selected_option: Option,
}

impl MainMenu {
  pub fn new() -> Self {
    MainMenu {
      selected_option: Option::SinglePlayer,
    }
  }

  pub fn draw(&self, rld: &mut RaylibDrawHandle) {
    let screen_width = rld.get_screen_width() as f32;
    let screen_height = rld.get_screen_height() as f32;

    let font_size = screen_height / 10.0;
    let font_size_big = screen_height / 4.0;

    rld.clear_background(Color::LIGHTGRAY);
    rld.draw_text(
      "RAYTRIS",
      (screen_width as i32 - rld.measure_text("RAYTRIS", font_size_big as i32)) / 2,
      screen_height as i32 / 2 - font_size_big as i32 - font_size as i32,
      font_size_big as i32,
      Color::RED,
    );

    let box_width = 8.0 * font_size;
    let box_height = 1.3 * font_size;
    let separation = 1.5 * font_size;

    for (idx, option) in Option::OPTIONS.iter().enumerate() {
      let s = option.into_str();
      let is_selected = *option == self.selected_option;
      let enclosing_box = Rectangle {
        x: (screen_width - box_width) / 2.0,
        y: (screen_height - box_height + font_size) / 2.0 + idx as f32 * separation,
        width: box_width,
        height: box_height,
      };

      rld.draw_rectangle_rec(
        enclosing_box,
        if is_selected {
          Color::SKYBLUE
        } else {
          Color::GRAY
        },
      );

      rld.draw_rectangle_lines_ex(
        enclosing_box,
        font_size / 10.0,
        if is_selected {
          Color::BLUE
        } else {
          Color::BLACK
        },
      );

      rld.draw_text(
        s,
        (screen_width as i32 - rld.measure_text(s, font_size as i32)) / 2,
        (screen_height / 2.0 + idx as f32 * separation) as i32,
        font_size as i32,
        if is_selected {
          Color::BLUE
        } else {
          Color::BLACK
        },
      );
    }
  }

  pub fn update(&mut self, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
      self.selected_option = self.selected_option.next();
    } else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
      self.selected_option = self.selected_option.prev();
    }
  }

  pub fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
  }

  pub fn selected(&self, rl: &RaylibHandle) -> Option {
    if rl.is_key_down(KeyboardKey::KEY_ENTER) {
      self.selected_option
    } else {
      Option::Exit
    }
  }
}
