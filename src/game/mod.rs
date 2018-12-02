use std::time::{ Instant, Duration };

use ggez::{
  Context,
  GameResult,
  graphics,
  event::{
    self,
    Keycode
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

pub struct GameState {
  window_size:   Size,
  window_rect:   Rect,
  input_manager: InputManager,
  level:         Option<Level>,
  running:       bool,
  last_update:   Instant,

  // TODO tmp
  song:          audio::Source
}

impl GameState {
  pub fn new(ctx: &mut Context, window_size: Size) -> GameResult<Self> {
    Ok(Self {
      window_size:   window_size.clone(),
      window_rect:   Rect::new(Point::new(0.0, 0.0), window_size, Origin::TopLeft),
      input_manager: InputManager::new(),
      level:         None,
      running:       true,
      last_update:   Instant::now(),

      // TODO tmp
      song:          audio::Source::new(ctx, ::join_str(res::AUDIO, &"title_edit.wav"))?
    })
  }

  pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.level = Some(Level::new(ctx, "pits")?);
    self.song.play();
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

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    let now = Instant::now();
    if !self.running || now - self.last_update < Duration::from_millis(UPDATE_INTERVAL_MS) {
      return Ok(());
    }

    if let Some(ref mut level) = self.level {
      level.keys_pressed(self.input_manager.keys());
      level.update(_ctx)?;
    }

    self.last_update = now;
    return Ok(());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);

    if let Some(ref mut level) = self.level {
      level.draw(ctx)?;
    }

    graphics::present(ctx);
    ::ggez::timer::yield_now();
    return Ok(());
  }
}
