mod main_menu;
mod settings;
use main_menu::MainMenu;
use raylib::{
  RaylibHandle, RaylibThread,
  color::Color,
  init,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::settings::{
  menu::{Menu as SettingsMenu, config},
  resolution::Resolution,
};

enum App {
  MainMenu(MainMenu),
  SettingsMenu(SettingsMenu),
}

impl App {
  fn draw(&self, rld: &mut RaylibDrawHandle) {
    match self {
      Self::MainMenu(main_menu) => main_menu.draw(rld),
      Self::SettingsMenu(settings_menu) => settings_menu.draw(rld),
    }
  }

  fn update(&mut self, rl: &mut RaylibHandle) {
    match self {
      Self::MainMenu(main_menu) => main_menu.update(rl),
      Self::SettingsMenu(settings_menu) => settings_menu.update(rl),
    }
  }

  fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    match self {
      Self::MainMenu(main_menu) => main_menu.should_stop_running(rl),
      Self::SettingsMenu(settings_menu) => settings_menu.should_stop_running(rl),
    }
  }
}

pub struct Raytris {
  app: App,
  should_stop_running: bool,
  rl: RaylibHandle,
  thread: RaylibThread,
}

impl Raytris {
  pub fn new() -> Self {
    let (mut rl, thread) = init().title("RAYTRIS").build();
    let resolution = config().resolution;
    let (width, height) = resolution.size();
    rl.set_window_size(width, height);
    if resolution == Resolution::Fullscreen {
      rl.toggle_fullscreen();
    }

    Self {
      app: App::MainMenu(MainMenu::new()),
      should_stop_running: false,
      rl,
      thread,
    }
  }

  fn handle_where_to_go(&mut self) {
    match &self.app {
      App::MainMenu(main_menu) => match main_menu.selected() {
        main_menu::Option::Exit => self.should_stop_running = true,
        main_menu::Option::Settings => self.app = App::SettingsMenu(SettingsMenu::new()),
        _ => (),
      },
      _ => self.app = App::MainMenu(MainMenu::new()),
    }
  }

  pub fn run(&mut self) {
    self.rl.set_target_fps(60);
    while !self.should_stop_running {
      self.app.update(&mut self.rl);
      if self.app.should_stop_running(&self.rl) {
        self.handle_where_to_go();
      }

      let mut d = self.rl.begin_drawing(&self.thread);
      d.clear_background(Color::LIGHTGRAY);
      self.app.draw(&mut d);
    }
  }
}
