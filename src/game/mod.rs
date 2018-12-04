use std::time::{ Instant, Duration };

use ggez::{
  Context,
  GameResult,
  graphics,
  event::{
    self,
    Keycode,
    MouseButton
  },
  audio
};

use noframe::geo::prelude::*;
use noframe::entity::{
  Entity,
  Movement
};
use noframe::input_manager::InputManager;
use noframe::camera::Camera;
use noframe::deltatime::Deltatime;

use settings::game::*;
use settings::res;
use level::Level;
use interactables::Interactable;
use menu::Menu;
use menu::MenuManager;
use menu::buttons::Button;
use menu::ButtonType;

enum Scene {
  Title,
  Ingame
}

pub struct GameState {
  window_size:   Size,
  window_rect:   Rect,
  input_manager: InputManager,
  level:         Option<Level>,
  menu_manager:  MenuManager,
  running:       bool,
  last_update:   Instant,
  scene:         Scene,

  // TODO tmp
  song:          audio::Source
}

impl GameState {
  pub fn new(ctx: &mut Context, window_size: Size) -> GameResult<Self> {
    let mut song = audio::Source::new(ctx, ::join_str(res::AUDIO, &"titletheme.wav"))?;
    song.set_repeat(true);
    Ok(Self {
      window_size:   window_size.clone(),
      window_rect:   Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft),
      input_manager: InputManager::new(),
      level:         None,
      running:       true,
      last_update:   Instant::now(),
      menu_manager:  MenuManager::new(ctx, window_size.clone()),
      scene:         Scene::Title,

      // TODO tmp
      song:          song
    })
  }

  pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.song.play();
    Ok(())
  }

  fn update_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(ref mut level) = self.level {
      level.keys_pressed(self.input_manager.keys_pressed());
      level.keys_down(self.input_manager.keys_down());
      level.keys_up(self.input_manager.keys_up());
      level.update(ctx)?;
    }
    Ok(())
  }

  fn update_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    // self.menu_manager.title.mouse_down(self.input_manager.mouse_down());
    // self.menu_manager.title.mouse_up(self.input_manager.mouse_up());
    if let Some(ButtonType::Start) = self.menu_manager.title.get_clicked() {
      self.start_game(ctx)?;
    }
    self.menu_manager.title.update()?;
    Ok(())
  }

  fn start_game(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level = Some(Level::new(ctx, self.window_size.clone(), "main")?);
    self.scene = Scene::Ingame;
    Ok(())
  }

  fn draw_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(ref mut level) = self.level {
      level.draw(ctx)?;
    };
    Ok(())
  }

  fn draw_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.menu_manager.title.draw(ctx)?;
    Ok(())
  }
}

impl event::EventHandler for GameState {
  fn key_down_event(&mut self,
                    ctx:     &mut Context,
                    keycode: Keycode,
                    _keymod: event::Mod,
                    repeat:  bool) {
    self.input_manager.key_down(keycode, _keymod, repeat);
    if let Keycode::Escape = keycode {
      ctx.quit().expect("Should quit Context");
    }
  }

  fn key_up_event(&mut self,
                  _ctx:    &mut Context,
                  keycode: Keycode,
                  _keymod: event::Mod,
                  repeat:  bool) {
    self.input_manager.key_up(keycode, _keymod, repeat);
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
    // self.input_manager.add_mouse_down(btn, x, y);
    match self.scene {
      Scene::Title  => self.menu_manager.title.mouse_down(x, y),
      Scene::Ingame => {
        if let Some(ref mut level) = self.level {
          level.mouse_down(x, y);
        }
      }
    }
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut Context, btn: MouseButton, x: i32, y: i32) {
    // self.input_manager.add_mouse_up(btn, x, y);
    // if let Scene::Title = self.scene {
    //   self.menu_manager.title.mouse_up(x, y);
    // }
  }

  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let now = Instant::now();
    if !self.running || now - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }

    match self.scene {
      Scene::Title  => self.update_menu(ctx)?,
      Scene::Ingame => self.update_ingame(ctx)?
    };

    self.input_manager.update();
    self.last_update = now;
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    match self.scene {
      Scene::Title  => self.draw_menu(ctx)?,
      Scene::Ingame => self.draw_ingame(ctx)?
    };

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
