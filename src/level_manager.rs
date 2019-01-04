use std::collections::hash_map::HashMap;

use ggez::{
  GameResult,
  Context,
  event::Keycode,
  audio,
};
use noframe::geo::prelude::*;

use level::Level;
use settings::level_manager::*;
use settings::res;
use animation::Animation;
use animation::Facing;
use score::Score;
use menu::pause::prelude::*;

pub struct LevelManager {
  level_index:      usize,
  level:            Option<Level>,
  level_names:      Vec<&'static str>,
  song:             Option<audio::Source>,
  song_names:       Vec<&'static str>,
  background:       Option<Animation>,
  window_size:      Size,
  scores:           HashMap<&'static str, Score>,
  paused:           bool,
  pause_menu:       PauseMenu,
  pub to_title:     bool,
}

impl LevelManager {
  pub fn new(ctx: &mut Context, window_size: Size) -> Self {
    Self {
      level_index: 0,
      level:       None,
      level_names: LEVEL_NAMES.to_vec(),
      song:        None,
      song_names:  SONG_NAMES.to_vec(),
      background:  None,
      window_size: window_size.clone(),
      scores:      HashMap::new(),
      paused:      false,
      pause_menu:  PauseMenu::new(ctx, window_size.clone()),
      to_title:    false,
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
    // Save the current level's score
    if let Some(level) = &mut self.level {
      let prev_level_index_opt = if self.level_index > 0 { Some(self.level_index - 1) } else { None };
      if let Some(prev_level_index) = prev_level_index_opt {
        if let Some(level_name) = self.level_names.get(prev_level_index) {
          self.scores.insert(level_name, level.score().clone());
        }
      }
    }

    // Load the next level
    if let Some(level_name) = self.level_names.get(self.level_index) {
      self.level = Some( Level::new(ctx, self.window_size.clone(), level_name)? );
    } else {
      self.level = None;
    }
    // Load audio
    let mut muted = false;
    if let Some(song) = &self.song {
      muted = song.paused();
      song.stop();
    }
    if let Some(song_name) = self.song_names.get(self.level_index) {
      let mut song = audio::Source::new(ctx, format!("{}{}.{}", res::AUDIO, song_name, AUDIO_FORMAT))?;
      song.set_volume(0.5);
      song.set_repeat(true);
      song.play()?;
      if muted { song.pause(); }
      self.song = Some( song );
    }
    // Load background animation
    self.background = new_background(ctx, self.level_index);
    if self.level.is_some() {
      self.level_index += 1;
    }
    Ok(())
  }

  pub fn keys_pressed(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_pressed(keys);
    }
  }

  pub fn keys_down(&mut self, ctx: &mut Context, keys: &Vec<Keycode>) {
    for &key in keys {
      match key {
        controls::MUTE  => self.toggle_mute(),
        controls::PAUSE => self.toggle_pause(),
        controls::RESET => self.reset(ctx).expect("Should reset level"),
        _ => ()
      }
    }

    if let Some(level) = &mut self.level {
      level.keys_down(keys);
    }
  }

  pub fn keys_up(&mut self, _ctx: &mut Context, keys: &Vec<Keycode>) {
    if let Some(level) = &mut self.level {
      level.keys_up(keys);
    }
  }

  pub fn mouse_down(&mut self, x: i32, y: i32) {
    if self.paused {
      self.pause_menu.mouse_down(x, y);
    }
    if let Some(level) = &mut self.level {
      level.mouse_down(x, y);
    }
  }

  pub fn mouse_drag(&mut self, xrel: i32, yrel: i32) {
    if let Some(level) = &mut self.level {
      level.camera_mut().move_by(&Point::new(xrel as NumType, yrel as NumType).inverted());
    }
  }

  fn toggle_mute(&mut self) {
    if let Some(song) = &self.song {
      if song.paused() {
        song.resume();
      } else {
        song.pause();
      }
    }
  }

  fn toggle_pause(&mut self) {
    if self.paused {
      if let Some(level) = &mut self.level {
        level.reset_dt();
      }
      self.paused = false;
    } else {
      self.paused = true;
    }
  }

  fn reset(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(level) = &mut self.level {
      level.reset(ctx)?;
    }
    self.paused = false;
    Ok(())
  }

  pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.update_pause_menu(ctx)?;
    self.update_level(ctx)?;
    Ok(())
  }

  fn update_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.paused { return Ok(()); }
    let mut next_level = false;
    if let Some(level) = &mut self.level {
      level.update(ctx)?;
      next_level = level.goto_next_level();
    }
    if next_level {
      self.next_level(ctx)?;
    }
    Ok(())
  }

  fn update_pause_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.pause_menu.resume {
      self.pause_menu.resume = false;
      self.toggle_pause();
    }
    if self.pause_menu.to_title {
      if self.level_index > 0 {
        self.level_index -= 1;
      }
      if let Some(song) = &mut self.song {
        song.stop();
      }
      self.pause_menu.to_title = false;
      self.paused = false;
      self.to_title = true;
    }
    if self.pause_menu.reset {
      self.pause_menu.reset = false;
      self.reset(ctx)?;
    }

    if self.paused {
      self.pause_menu.update()?;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.draw_level(ctx)?;
    if self.paused {
      self.draw_pause_menu(ctx)?;
    }
    Ok(())
  }

  fn draw_level(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(bg) = &self.background {
      bg.draw(ctx, &Point::new(0.0, 0.0), &self.window_size, &Facing::Right)?;
    }
    if let Some(level) = &mut self.level {
      level.draw(ctx)?;
    }
    Ok(())
  }

  fn draw_pause_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.pause_menu.draw(ctx)?;
    Ok(())
  }
}

fn new_background(ctx: &mut Context, n: usize) -> Option<Animation> {
  match n {
    _ => Some(Animation::new(
        ctx,
        vec![::join_str(res::BACKGROUND_IMAGES, "default.png")],
        vec![1000]
        ))
  }
}
