use ggez::{
  GameResult,
  Context,
  event::Keycode
};
use noframe::geo::prelude::*;

use level::Level;
use settings::levels::*;

pub struct LevelManager {
  level_index: usize,
  level:       Option<Level>,
  level_names: Vec<&'static str>,
  window_size: Size
}

impl LevelManager {
  pub fn new(window_size: Size) -> Self {
    Self {
      level_index: 0,
      level: None,
      level_names: LEVEL_NAMES.to_vec(),
      window_size
    }
  }

  pub fn level(&mut self) -> Option<&mut Level> {
    if let Some(level) = &mut self.level {
      Some(level)
    } else {
      None
    }
  }

  pub fn next_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(level_name) = self.level_names.get(self.level_index) {
      self.level = Some( Level::new(ctx, self.window_size.clone(), level_name)? );
      self.level_index += 1;
    } else {
      self.level = None;
    }
    Ok(())
  }

  pub fn keys_pressed(&mut self, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_pressed(keys);
    }
  }

  pub fn keys_down(&mut self, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_down(keys);
    }
  }

  pub fn keys_up(&mut self, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_up(keys);
    }
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut next_level = false;
    if let Some(level) = &mut self.level {
      level.update(ctx)?;
      next_level = level.next_level();
    }
    if next_level {
      self.next_level(ctx)?;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(level) = &mut self.level {
      level.draw(ctx)?;
    }
    Ok(())
  }
}
