use raylib::{
  RaylibHandle,
  color::Color,
  ffi::{KeyboardKey, Rectangle},
  prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Option {
  SinglePlayer,
  TwoPlayer,
  Settings,
  Exit,
}

const OPTIONS: [Option; 4] = [
  Option::SinglePlayer,
  Option::TwoPlayer,
  Option::Settings,
  Option::Exit,
];
impl Option {
  fn name(self) -> &'static str {
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
      Self::Settings => Self::Exit,
      Self::Exit => Self::SinglePlayer,
    }
  }

  fn prev(self) -> Self {
    match self {
      Self::SinglePlayer => Self::Exit,
      Self::TwoPlayer => Self::SinglePlayer,
      Self::Settings => Self::TwoPlayer,
      Self::Exit => Self::SinglePlayer,
    }
  }
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
    let font_size = screen_height / 12.0;
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
    let separation = 1.5 * font_size;
    let box_height = 1.3 * font_size;
    for (idx, &option) in OPTIONS.iter().enumerate() {
      let name = option.name();
      let is_selected = option == self.selected_option;
      let enclosing_box = Rectangle {
        x: (screen_width - box_width) / 2.0,
        y: (screen_height - box_height + font_size) / 2.0 + idx as f32 * separation,
        width: box_width,
        height: box_height,
      };
      let (inner_color, outer_color) = if is_selected {
        (Color::BLUE, Color::SKYBLUE)
      } else {
        (Color::BLACK, Color::GRAY)
      };
      rld.draw_rectangle_rec(enclosing_box, outer_color);
      rld.draw_rectangle_lines_ex(enclosing_box, font_size / 10.0, inner_color);
      let name_x = (screen_width as i32 - rld.measure_text(name, font_size as i32)) / 2;
      let name_y = (screen_height / 2.0 + idx as f32 * separation) as i32;
      rld.draw_text(name, name_x, name_y, font_size as i32, inner_color);
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
    rl.is_key_pressed(KeyboardKey::KEY_ENTER)
  }

  pub fn selected(&self) -> Option {
    self.selected_option
  }
}
