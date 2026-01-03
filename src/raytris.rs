mod main_menu;
use main_menu::MainMenu;
use raylib::{
  RaylibHandle, RaylibThread,
  color::Color,
  init,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

enum App {
  MainMenu(MainMenu),
}

impl App {
  fn draw(&self, rld: &mut RaylibDrawHandle) {
    match self {
      Self::MainMenu(main_menu) => main_menu.draw(rld),
    }
  }

  fn update(&mut self, rl: &mut RaylibHandle) {
    match self {
      Self::MainMenu(main_menu) => main_menu.update(rl),
    }
  }

  fn should_stop_running(&self, rl: &RaylibHandle) -> bool {
    match self {
      Self::MainMenu(main_menu) => main_menu.should_stop_running(rl),
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
    let (rl, thread) = init().size(1280, 720).title("RAYTRIS").build();
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
        _ => {}
      },
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
