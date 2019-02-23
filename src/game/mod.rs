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
use menu::title_menu_manager::prelude::*;
use menu::buttons::ButtonType;
use frames_counter::FramesCounter;

enum Scene {
  Title,
  Ingame,
}

pub struct GameState {
  window_size:   Size,
  window_rect:   Rect,
  input_manager: InputManager,
  level_manager: LevelManager,
  menu_manager:  TitleMenuManager,
  running:       bool,
  last_update:   Instant,
  scene:         Scene,
  title_song:    Option<audio::Source>,
  fps:           FramesCounter,
  ups:           FramesCounter,

  last_log:      Instant,
}

impl GameState {
  pub fn new(ctx: &mut Context, window_size: Size) -> GameResult<Self> {
    Ok(Self {
      window_size:   window_size.clone(),
      window_rect:   Rect::new(Point::new(0.0, 0.0), window_size.clone(), Origin::TopLeft),
      input_manager: InputManager::new(),
      level_manager: LevelManager::new(ctx, window_size.clone()),
      running:       true,
      last_update:   Instant::now(),
      menu_manager:  TitleMenuManager::new(ctx, window_size.clone())?,
      scene:         Scene::Title,
      title_song:    None,
      fps:           FramesCounter::new(),
      ups:           FramesCounter::new(),

      last_log:      Instant::now(),
    })
  }

  pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.play_song(ctx)?;
    Ok(())
  }

  fn play_song(&mut self, ctx: &mut Context) -> GameResult<()> {
    if !MUTED {
      if let Some(song) = &mut self.title_song {
        song.stop();
      }
      let mut title_song = audio::Source::new(ctx, ::join_str(res::AUDIO, &"titletheme.ogg"))?;
      title_song.set_volume(VOLUME);
      title_song.set_repeat(true);
      title_song.play()?;
      self.title_song = Some(title_song);
    }
    Ok(())
  }

  fn stop_song(&mut self) {
    if let Some(song) = &mut self.title_song {
      song.stop();
    }
    self.title_song = None;
  }

  fn update_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.level_manager.to_title {
      self.play_song(ctx)?;
      self.level_manager.to_title = false;
      self.scene = Scene::Title;
    }
    self.level_manager.keys_pressed(ctx, self.input_manager.keys_pressed());
    self.level_manager.keys_down(ctx, self.input_manager.keys_down());
    self.level_manager.keys_up(ctx, self.input_manager.keys_up());
    self.level_manager.update(ctx)?;
    Ok(())
  }

  fn update_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.level_manager.beat_game {
      self.menu_manager.show_level_select();
    }
    let mut start_game   = false;
    let mut quit         = false;
    if let Some(clicked) = self.menu_manager.get_clicked() {
      match clicked {
        ButtonType::TitleStart => start_game   = true,
        ButtonType::TitleQuit  => quit         = true,
        _ => ()
      }
    }
    if let Some(level_index) = self.menu_manager.load_level {
      self.menu_manager.load_level = None;
      self.start_level(ctx, level_index)?;
    }
    if start_game {
      self.start_game(ctx)?;
    } else if quit {
      ctx.quit()?;
    }
    self.menu_manager.update()?;
    Ok(())
  }

  fn start_game(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.stop_song();
    self.level_manager.next_level(ctx)?;
    self.scene = Scene::Ingame;
    Ok(())
  }

  fn start_level(&mut self, ctx: &mut Context, level_index: usize) -> GameResult<()> {
    self.stop_song();
    self.level_manager.load_level(ctx, level_index)?;
    self.scene = Scene::Ingame;
    Ok(())
  }

  fn draw_ingame(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level_manager.draw(ctx)?;
    Ok(())
  }

  fn draw_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.menu_manager.draw(ctx)?;
    Ok(())
  }

  // TODO: TEMPORARY, FOR DEBUGGING
  fn update_debug(&mut self) {
    let now = Instant::now();
    if now - self.last_log > Duration::from_secs(1) {
      println!("{} UPS / {} FPS",
               self.ups.avg(), self.fps.avg());
      self.last_log = now;
    }
    // TODO: TEMPORARY ARTIFICIAL LAG!!!
    for keycode in self.input_manager.keys_pressed() {
      if let Keycode::O = keycode {
        std::thread::sleep(Duration::new(0, 50_000_000));
      }
    }
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
    if let Scene::Title = self.scene {
      match keycode {
        Keycode::Return => self.start_game(ctx).expect("Should start game"),
        Keycode::L      => self.menu_manager.show_level_select(),  // TODO: TEMPORARY!!!
        _               => (),
      }
    }
  }

  fn key_up_event(&mut self,
                  _ctx:    &mut Context,
                  keycode: Keycode,
                  _keymod: event::Mod,
                  repeat:  bool) {
    self.input_manager.key_up(keycode, _keymod, repeat);
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _btn: MouseButton, x: i32, y: i32) {
    // self.input_manager.add_mouse_down(btn, x, y);
    match self.scene {
      Scene::Title  => self.menu_manager.mouse_down(x, y),
      Scene::Ingame => self.level_manager.mouse_down(x, y),
    }
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

    // TODO: TEMPORARY, FOR DEBUGGING
    self.update_debug();

    match self.scene {
      Scene::Title  => self.update_menu(ctx)?,
      Scene::Ingame => self.update_ingame(ctx)?,
    };

    self.input_manager.update();
    self.last_update = now;
    self.ups.update();
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    match self.scene {
      Scene::Title  => self.draw_menu(ctx)?,
      Scene::Ingame => self.draw_ingame(ctx)?,
    };

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    self.fps.update();
    return Ok(());
  }
}
