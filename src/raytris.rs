mod gameplay;
mod main_menu;
mod settings;

use main_menu::MainMenu;
use raylib::{
  RaylibHandle, RaylibThread, init,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::raytris::{
  gameplay::{BACKGROUND_COLOR, single_player::SinglePlayer, two_player::TwoPlayer},
  settings::{
    menu::{Menu as SettingsMenu, config},
    resolution::Resolution,
  },
};

enum App {
  MainMenu(MainMenu),
  SettingsMenu(SettingsMenu),
  SinglePlayer(SinglePlayer),
  TwoPlayer(TwoPlayer),
}

impl App {
  fn draw(&self, rld: &mut RaylibDrawHandle) {
    match self {
      App::MainMenu(main_menu) => main_menu.draw(rld),
      App::SettingsMenu(settings_menu) => settings_menu.draw(rld),
      App::SinglePlayer(single_player) => single_player.draw(rld),
      App::TwoPlayer(two_player) => two_player.draw(rld),
    }
  }

  fn update(&mut self, rl: &mut RaylibHandle) {
    match self {
      App::MainMenu(main_menu) => main_menu.update(rl),
      App::SettingsMenu(settings_menu) => settings_menu.update(rl),
      App::SinglePlayer(single_player) => single_player.update(rl),
      App::TwoPlayer(two_player) => two_player.update(rl),
    }
  }

  fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    match self {
      App::MainMenu(main_menu) => main_menu.should_stop_running(rl),
      App::SettingsMenu(settings_menu) => settings_menu.should_stop_running(rl),
      App::SinglePlayer(single_player) => single_player.should_stop_running(rl),
      App::TwoPlayer(two_player) => two_player.should_stop_running(rl),
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
    use main_menu::Option;
    match &self.app {
      App::MainMenu(main_menu) => {
        self.app = match main_menu.selected() {
          Option::Exit => {
            self.should_stop_running = true;
            return;
          }
          Option::Settings => App::SettingsMenu(SettingsMenu::new()),
          Option::SinglePlayer => {
            App::SinglePlayer(SinglePlayer::new(config().handling_settings, &self.rl))
          }
          Option::TwoPlayer => App::TwoPlayer(TwoPlayer::new(
            config().handling_settings,
            config().handling_settings,
            &self.rl,
          )),
        }
      }
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
      d.clear_background(BACKGROUND_COLOR);
      self.app.draw(&mut d);
    }
  }
}
