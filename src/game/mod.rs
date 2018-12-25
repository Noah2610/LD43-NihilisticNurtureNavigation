use std::time::{ Instant, Duration };

use ggez::{
  Context,
  GameResult,
  graphics,
  event::{
    self,
    Keycode,
    MouseButton,
    MouseState
  },
  audio
};

use noframe::geo::prelude::*;
use noframe::input_manager::InputManager;

use settings::game::*;
use settings::res;
use level_manager::LevelManager;
use menu::Menu;
use menu::MenuManager;
use menu::ButtonType;

enum Scene {
  Title,
  Ingame
}

pub struct GameState {
  window_size:   Size,
  window_rect:   Rect,
  input_manager: InputManager,
  level_manager: LevelManager,
  menu_manager:  MenuManager,
  running:       bool,
  last_update:   Instant,
  scene:         Scene,
  title_song:    audio::Source
}

impl GameState {
  pub fn new(ctx: &mut Context, window_size: Size) -> GameResult<Self> {
    let mut title_song = audio::Source::new(ctx, ::join_str(res::AUDIO, &"titletheme.ogg"))?;
    title_song.set_volume(0.5);
    title_song.set_repeat(true);
    Ok(Self {
      window_size:   window_size.clone(),
      window_rect:   Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft),
      input_manager: InputManager::new(),
      level_manager: LevelManager::new(window_size.clone()),
      running:       true,
      last_update:   Instant::now(),
      menu_manager:  MenuManager::new(ctx, window_size.clone()),
      scene:         Scene::Title,
      title_song:    title_song
    })
  }

  pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.title_song.play();
    Ok(())
  }

  fn update_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level_manager.keys_pressed(self.input_manager.keys_pressed());
    self.level_manager.keys_down(self.input_manager.keys_down());
    self.level_manager.keys_up(self.input_manager.keys_up());
    self.level_manager.update(ctx)?;
    Ok(())
  }

  fn update_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(ButtonType::Start) = self.menu_manager.title.get_clicked() {
      self.start_game(ctx)?;
    }
    self.menu_manager.title.update()?;
    Ok(())
  }

  fn start_game(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level_manager.next_level(ctx)?;
    self.title_song.stop();
    self.scene = Scene::Ingame;
    Ok(())
  }

  fn draw_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level_manager.draw(ctx)?;
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
        if let Some(ref mut level) = self.level_manager.level() {
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

  fn mouse_motion_event(&mut self, _ctx: &mut Context, state: MouseState, _x: i32, _y: i32, xrel: i32, yrel: i32) {
    if state.left() || state.right() {
      self.level_manager.mouse_drag(xrel, yrel);
    }
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
